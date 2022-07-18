use crate::entrypoint;

#[test]
#[cfg(feature = "ui-tests")]
fn check_entrypoint_exists() {
    println!("This requires a Desktop Environment up and running! Tests will fail otherwise.");
    entrypoint();
}

// Example for UI tests:
// #[test]
// #[cfg(feature = "ui-tests")]
// fn some_ui_test() {
//     assert!(true);
// }
