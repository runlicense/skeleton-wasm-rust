use std::fs;

fn main() {
    println!("cargo:rerun-if-changed=runlicense.json");

    let json = fs::read_to_string("runlicense.json")
        .expect("Failed to read runlicense.json");
    let parsed: serde_json::Value = serde_json::from_str(&json)
        .expect("Failed to parse runlicense.json");
    let namespace = parsed["namespace"]
        .as_str()
        .expect("Missing 'namespace' in runlicense.json");
    let product_name = parsed["product_name"]
        .as_str()
        .expect("Missing 'product_name' in runlicense.json");

    println!("cargo:rustc-env=RUNLICENSE_NAMESPACE={namespace}");
    println!("cargo:rustc-env=RUNLICENSE_PRODUCT_NAME={product_name}");
}
