use crate::main_rs;

#[test]
fn check_main_rs_exists() {
    println!("This requires a Desktop Environment up and running! Tests will fail otherwise.");
    main_rs();
}

// Example for UI tests:
// #[test]
// #[cfg(feature = "ui-tests")]
// fn some_ui_test() {
//     assert!(true);
// }
