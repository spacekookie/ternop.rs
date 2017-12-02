//! Small unit testing file for ternop

#[test]
fn ternary_testing() {
    let a = ternary![true, true, false];
    assert!(a == true);

    let b = ternary![a, 1, 2];
    assert!(b == 1);

    let c = ternary![b == 2, "yes", "noep!"];
    assert!(c == "noep!");
}
