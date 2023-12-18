declare namespace wasm_bindgen {
	/* tslint:disable */
	/* eslint-disable */
	/**
	*/
	export function start(): void;
	/**
	*/
	export class WebSocketWrapper {
	  free(): void;
	/**
	* @param {string} url
	*/
	  constructor(url: string);
	/**
	* @returns {number}
	*/
	  get_status(): number;
	/**
	* @returns {number}
	*/
	  get_buffered_amount(): number;
	/**
	* @param {number} size
	*/
	  set_file_size(size: number): void;
	/**
	* @returns {number}
	*/
	  get_file_size(): number;
	/**
	* @param {File} file
	* @returns {Promise<void>}
	*/
	  upload_file(file: File): Promise<void>;
	/**
	* @param {File} file
	* @returns {string}
	*/
	  sha256_file_sync(file: File): string;
	}
	
}

declare type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

declare interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_websocketwrapper_free: (a: number) => void;
  readonly start: () => void;
  readonly websocketwrapper_new: (a: number, b: number, c: number) => void;
  readonly websocketwrapper_get_status: (a: number) => number;
  readonly websocketwrapper_get_buffered_amount: (a: number) => number;
  readonly websocketwrapper_set_file_size: (a: number, b: number) => void;
  readonly websocketwrapper_get_file_size: (a: number) => number;
  readonly websocketwrapper_upload_file: (a: number, b: number) => number;
  readonly websocketwrapper_sha256_file_sync: (a: number, b: number, c: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
  readonly _dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h43c9e3e836ce7a3c: (a: number, b: number, c: number) => void;
  readonly _dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__he0825dd47d91b646: (a: number, b: number) => void;
  readonly wasm_bindgen__convert__closures__invoke1_mut__h3d98922384d39600: (a: number, b: number, c: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly wasm_bindgen__convert__closures__invoke2_mut__hfbd842a2a1d9464a: (a: number, b: number, c: number, d: number) => void;
  readonly __wbindgen_start: () => void;
}

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
declare function wasm_bindgen (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
