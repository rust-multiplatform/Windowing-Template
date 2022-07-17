use crate::entrypoint;

#[test]
fn check_entrypoint_exists() {
    println!("This requires a Desktop Environment up and running! Tests will fail otherwise.");
    entrypoint();
}
