use crate::main;

#[test]
fn check_main_exists() {
    println!("This requires a Desktop Environment up and running! Tests will fail otherwise.");
    main();
}
