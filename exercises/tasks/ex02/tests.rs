mod starter;
use starter::*;
#[test]
fn normal_cases() {
    assert!((fahrenheit(100.0) - 212.0).abs() < 1e-9);
}
#[test]
fn edge_cases() {
    assert!((fahrenheit(-40.0) + 40.0).abs() < 1e-9);
    assert!((fahrenheit(0.5) - 32.9).abs() < 1e-9);
}
