use wasm_bindgen_futures::wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn activate() {
    wasm_bindgen_futures::spawn_local(async move {
        log("[WASM] Validating license...");
        let valid = runlicense_sdk_webassembly_rust::verify_license_from_path!(env!("RUNLICENSE_NAMESPACE")).await.is_ok();
        log(&format!("[WASM] License valid: {valid}"));

        if valid {
            log("[WASM]  __________________________ ");
            log("[WASM] < License activated! Moo! >");
            log("[WASM]  -------------------------- ");
            log("[WASM]         \\   ^__^             ");
            log("[WASM]          \\  (oo)\\_______     ");
            log("[WASM]             (__)\\       )\\/\\ ");
            log("[WASM]                 ||----w |    ");
            log("[WASM]                 ||     ||    ");
        } else {
            log("[WASM]  __________________________ ");
            log("[WASM] < License NOT activated! NOOOOO!! >");
            log("[WASM]  -------------------------- ");
        }
    });
}
