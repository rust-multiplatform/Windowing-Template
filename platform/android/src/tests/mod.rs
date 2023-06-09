use crate::android_main;

#[test]
#[cfg(feature = "ui-tests")]
fn check_main_exists() {
    println!("This requires a Desktop Environment up and running! Tests will fail otherwise.");
    android_main();
}
