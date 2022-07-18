use crate::main;

#[test]
fn check_main_exists() {
    println!("This requires a Desktop Environment up and running! Tests will fail otherwise.");
    main();
}

// Example for UI tests:
// #[test]
// #[cfg(feature = "ui-tests")]
// fn some_ui_test() {
//     assert!(true);
// }
