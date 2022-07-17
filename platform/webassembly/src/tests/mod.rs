use crate::wasm_entry;

#[test]
fn check_wasm_entry_exists() {
    println!("This requires a Desktop Environment up and running! Tests will fail otherwise.");
    wasm_entry();
}
