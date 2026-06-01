#[test]
fn blocks_non_success_pointer() {
    let mut pointer = fixtures::pointer();
    pointer.status = "partial".to_owned();
    assert!(validate_pointer(&pointer, 0, 1000).is_err());
}

#[test]
fn accepts_pointer_covering_requested_window() {
    assert!(validate_pointer(&fixtures::pointer(), 1_000, 2_000).is_ok());
}

#[test]
fn rejects_pointer_outside_requested_window() {
    assert!(validate_pointer(&fixtures::pointer(), 900_000, 901_000).is_err());
}
