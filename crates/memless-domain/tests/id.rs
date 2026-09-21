use memless_domain::{Id, Scalar};

#[test]
fn builds_an_id_from_a_text_scalar() {
    assert_eq!(Id::from_scalar(Scalar::Text("a".to_string())), Some(Id::Text("a".to_string())));
}

#[test]
fn builds_an_id_from_an_integer_scalar() {
    assert_eq!(Id::from_scalar(Scalar::Integer(5)), Some(Id::Integer(5)));
}

#[test]
fn rejects_a_decimal_scalar() {
    assert_eq!(Id::from_scalar(Scalar::Decimal(1.5)), None);
}

#[test]
fn rejects_a_boolean_scalar() {
    assert_eq!(Id::from_scalar(Scalar::Boolean(true)), None);
}

#[test]
fn gives_equal_ids_for_the_same_text() {
    assert_eq!(Id::Text("x".to_string()), Id::Text("x".to_string()));
}

#[test]
fn renders_an_id_by_delegating_to_scalar_rendering() {
    assert_eq!(Id::Text("5".to_string()).to_string(), "\"5\"");
    assert_eq!(Id::Integer(5).to_string(), "5");
}
