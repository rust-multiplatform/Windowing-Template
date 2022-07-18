use crate::wasm_entry;

#[test]
fn check_wasm_entry_exists() {
    println!("This requires a Desktop Environment up and running! Tests will fail otherwise.");
    wasm_entry();
}

// Example for UI tests:
// #[test]
// #[cfg(feature = "ui-tests")]
// fn some_ui_test() {
//     assert!(true);
// }
