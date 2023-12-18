use std::io::{Read};
use std::{panic};
use lazy_static::lazy_static;
use serde::{Serialize, Deserialize, Serializer, Deserializer};
use wasm_bindgen::prelude::*;
use wasm_bindgen_file_reader::WebSysFile;
use wasm_bindgen_futures::JsFuture;
use web_sys::{console,File};
use web_sys::{MessageEvent, WebSocket,Event};

thread_local! {
    static FILE_READER_SYNC: web_sys::FileReaderSync = web_sys::FileReaderSync::new().expect("failed to create FileReaderSync. is this a web worker context?");
}


#[derive(Serialize, Deserialize)]
struct UploadMsg {
    #[serde(rename = "msg_type")]
    msg_type: i32,
    #[serde(rename = "msg")]
    msg: String,
    #[serde(rename = "filename")]
    filename: String,
    #[serde(rename = "object_name")]
    object_name: String,
}



#[derive(Debug,Serialize,Deserialize)]
struct UploadChunk {
    #[serde(rename = "type")]
    m_type: i32,
    payload: Vec<u8>,
}

impl UploadChunk {
    fn to_bytes(&self) -> Vec<u8> {
        let mut b = Vec::with_capacity(1 + self.payload.len());

        // 将 m_type 转换为一个字节并添加到向量中
        // 注意：这里假设 m_type 位于 0-255 的范围内
        b.push(self.m_type as u8);

        // 将 payload 的内容附加到向量中
        b.extend_from_slice(&self.payload);

        b
    }
}



#[wasm_bindgen]
pub struct WebSocketWrapper {
    ws: WebSocket,
    file_size :u32,
}


#[wasm_bindgen]
extern "C" {
    // 引入 JavaScript 的 `setTimeout` 函数
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    fn handleWebSocketClose();

    #[wasm_bindgen(js_name = sleep)]
    fn js_sleep(ms: i32) -> js_sys::Promise;
}

// 设置你想要维持的缓冲区大小
const MAX_BUFFERED_AMOUNT: u32 = 200 * 1024 * 1024; // 200 MB
const CHUNK_SIZE: u32 = 1024 * 1024; // 假设你的 chunk size 是 1 MB

#[wasm_bindgen(start)]
pub fn start() {
    wasm_logger::init(wasm_logger::Config::default());
    // Set panic hook so we get backtrace in console
    let next_hook = panic::take_hook();
    panic::set_hook(Box::new(move |info| {
        log::error!("PANIC: {}", &info.to_string());
        next_hook(info);
    }));

    // Logging
    log::debug!("Logger enabled, upload file from Rust!");
}

fn calculate_progress(total_size: usize, uploaded_size: usize) -> f64 {
    if total_size == 0 {
        return 0.0; // 避免除以零
    }
    (uploaded_size as f64 / total_size as f64) * 100.0
}

// 在 Rust 中实现异步的 sleep 方法
pub async fn sleep(ms: i32) {
    let promise = js_sleep(ms);
    JsFuture::from(promise).await.unwrap();
}


#[wasm_bindgen]
impl WebSocketWrapper {
    // 创建一个新的 WebSocket 连接
    #[wasm_bindgen(constructor)]
    pub fn new(url: &str) -> Result<WebSocketWrapper, JsValue> {
        let ws;
        if !url.contains("ws://"){
            ws = WebSocket::new("ws://1.1.1.1:10000/api/v1/object/upload_stream")?;
        }else{
            ws = WebSocket::new(url)?;
        }
        log::debug!("new WebSocketWrapper");
        Ok(WebSocketWrapper { ws,file_size:0 })
    }

    pub fn get_status(&mut self)->u32{
        let test_progress  = calculate_progress(self.get_file_size().try_into().unwrap(), self.ws.buffered_amount() as usize) as u32;
        log::debug!("call get_status {:?}",100 - test_progress);
        100 - test_progress
    }

    pub fn get_buffered_amount(&mut self)->u32{
        self.ws.buffered_amount()
    }
    // 新增设置文件大小的方法
    pub fn set_file_size(&mut self, size: u32) {
        self.file_size = size;
    }

    // 新增获取文件大小的方法
    pub fn get_file_size(&self) -> u32 {
        self.file_size
    }
    pub async fn upload_file(&mut self,file: File)-> Result<(), JsValue>  {
        let ws_cloned = self.ws.clone();
        // let ws = WebSocket::new("ws://192.168.2.20:20080/ws").unwrap();
        let msg = UploadMsg {
            msg_type: 4,
            msg: "-".into(),
            filename: "-".into(), // 修改为适当的文件名
            object_name: "-".into() // 修改为适当的对象名
        };

        let data = serde_json::to_vec(&msg).unwrap();
        // 文件基本信息
        let meta_chunk  = UploadChunk {
            m_type: 1,
            payload:data.clone(),
        };

        // 文件基本信息
        let done_chunk  = UploadChunk {
            m_type: 4,
            payload:data.clone(),
        };

        let file_size = file.size() as usize;

        let onmessage_callback = Closure::<dyn FnMut(_)>::new(move |e: MessageEvent| {
            console::log_1(&format!("onmessage data: {:?}", e).into());
            // 获取消息内容
        });

        ws_cloned.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
        onmessage_callback.forget();

        let ws_cloned2  = ws_cloned.clone();
        let ws_cloned3  = ws_cloned.clone();
        self.set_file_size(file_size as u32);
        let onopen_callback = Closure::<dyn FnMut()>::new(move || {
            // meta
            ws_cloned3.send_with_u8_array(&meta_chunk.to_bytes()).unwrap();

            let mu_file = file.clone();
            let mut wf = WebSysFile::new(mu_file);

            let mut buffer = vec![0; CHUNK_SIZE as usize];
            let mut count: u64 = 0;

            while let Ok(n) = wf.read(&mut buffer[..]) {
                if n == 0 {
                    // Probably end of file
                    break;
                }
                let rest = &buffer[0..n];

                let up_chunk = UploadChunk {
                    m_type: 2,
                    payload: Vec::from(rest),
                };
                match  ws_cloned3.send_with_u8_array(&up_chunk.to_bytes()) {
                    Ok(_) => {
                        log::debug!("send count{:?}",count);
                        log::debug!("buffered_amount  {:?}",ws_cloned3.buffered_amount());
                        // while 1==1{
                        //     // 100 - 200
                        //     let sendbuffer_count = ws_cloned3.buffered_amount();
                        //     if sendbuffer_count > MAX_BUFFERED_AMOUNT{
                        //         log::debug!("send count{:?}",count);
                        //         log::debug!("buffered_amount  {:?}",sendbuffer_count);
                        //         continue;
                        //     }else{
                        //         log::debug!("happy continue {:?}",count);
                        //         break
                        //     }
                        // }
                    }
                    Err(e) => {
                        // 发送数据失败
                        console::log_1(&format!("Error sending data: {:?}", e).into());
                    }
                }
                //TODO worker postmessage
                count += n as u64;
            }
            //done 收尾
            ws_cloned.send_with_u8_array(&done_chunk.to_bytes()).unwrap();

        });

        ws_cloned2.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
        onopen_callback.forget();

        let onerror = Closure::wrap(Box::new(move |e: JsValue| {
            console::log_1(&format!("WebSocket error: {:?}", e).into());
        }) as Box<dyn FnMut(_)>);

        ws_cloned2.set_onerror(Some(onerror.as_ref().unchecked_ref()));
        onerror.forget();

        let on_close= Closure::wrap(Box::new(move |e: MessageEvent| {
            console::log_1(&format!("on_close data: {:?}", e).into());
            handleWebSocketClose();
        }) as Box<dyn FnMut(_)>);

        ws_cloned2.set_onclose(Some(on_close.as_ref().unchecked_ref()));
        on_close.forget();

        Ok(())
    }

    pub fn sha256_file_sync(&mut self,file: web_sys::File) -> String {
        use sha2::{Digest, Sha256};
        const BUF_SIZE: usize = 1024 * 1024;

        let mut wf = WebSysFile::new(file);
        let mut hasher = Sha256::new();

        let mut buffer = vec![0; BUF_SIZE];
        let mut count: u64 = 0;
        while let Ok(n) = wf.read(&mut buffer[..]) {
            if n == 0 {
                // Probably end of file
                break;
            }

            let rest = &buffer[0..n];
            hasher.update(&rest);
            count += n as u64;
        }

        // read hash digest and consume hasher
        let result = hasher.finalize();

        // Return sha256 hash in hex and number of bytes read
        format!("{} ({} bytes)", hex::encode(result),count)
    }

}
