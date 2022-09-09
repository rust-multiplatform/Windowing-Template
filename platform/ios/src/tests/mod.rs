use crate::main_rs;

#[test]
#[cfg(feature = "ui-tests")]
fn check_main_rs_exists() {
    println!("This requires a Desktop Environment up and running! Tests will fail otherwise.");
    main_rs();
}
