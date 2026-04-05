use wasm_bindgen_futures::wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

fn cowsay(valid: bool) {
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
}

#[wasm_bindgen(js_name = "activateFromLicenseFile")]
pub fn activate_from_license_file() {
    wasm_bindgen_futures::spawn_local(async move {
        log("[WASM] Validating license from file...");
        let valid = runlicense_sdk_webassembly_rust::verify_license_from_path!(env!("RUNLICENSE_NAMESPACE")).await.is_ok();
        log(&format!("[WASM] License valid: {valid}"));
        cowsay(valid);
    });
}

#[wasm_bindgen(js_name = "activateFromLicenseString")]
pub fn activate_from_license_string(license_json: &str) {
    let license_json = license_json.to_string();
    wasm_bindgen_futures::spawn_local(async move {
        log("[WASM] Validating license from string...");
        let valid = runlicense_sdk_webassembly_rust::verify_license!(&license_json).await.is_ok();
        log(&format!("[WASM] License valid: {valid}"));
        cowsay(valid);
    });
}
