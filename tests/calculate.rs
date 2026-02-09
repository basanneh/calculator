use calculator::calculate;

#[test]
fn add_works() {
    assert_eq!(calculate(2.0, "+", 3.0).unwrap(), 5.0);
}

#[test]
fn division_by_zero_errors() {
    assert!(calculate(10.0, "/", 0.0).is_err());
}
