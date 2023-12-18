importScripts("wasm_websocket_upload_big_file.js");

function handleWebSocketClose() {
    console.log("上传完成");
    // 这里可以执行任何您需要的关闭处理逻辑
}
function sleep(ms) {
    return new Promise(resolve => setTimeout(resolve, ms));
}

onmessage = async function(e) {
    console.log("Message received from main script", e.data);
    let wasm_upload_file = await Rust.wasm_upload_file;
    let workerResult = new wasm_upload_file.WebSocketWrapper("");
    if (e.data.action=="send"){
        setInterval(function (){
            workerResult.get_status()
        }, 500, );
        let upload_result  =  workerResult.upload_file(e.data.file)
        console.log("send Posting message back to main script");
        postMessage( upload_result);
    }
    if (e.data.action=="md5"){
        let upload_result  =  workerResult.sha256_file_sync(e.data.file)
        console.log("md5 Posting message back to main script");
        postMessage( upload_result);
    }

};