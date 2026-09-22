mod starter;
#[test]
fn doubles() {
    assert_eq!(starter::double(3), 6);
    assert_eq!(starter::double(i32::MAX), 4_294_967_294);
}
