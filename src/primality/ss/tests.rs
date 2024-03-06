use super::SolovayStrassenTest;

use ibig::ubig;

#[test]
fn prime() {
    assert!(ubig!(5).solovay_strassen_test(1000) >= 0.75);
    assert!(ubig!(13).solovay_strassen_test(1000) >= 0.75);
    assert!(ubig!(29).solovay_strassen_test(1000) >= 0.75);
    assert!(ubig!(53).solovay_strassen_test(1000) >= 0.75);
}

#[test]
fn not_prime() {
    assert!(ubig!(20).solovay_strassen_test(1000) < 0.75);
    assert!(ubig!(100).solovay_strassen_test(1000) < 0.75);
    assert!(ubig!(200).solovay_strassen_test(1000) < 0.75);
}

#[test]
fn huge_prime() {
    assert!(ubig!(659555973512315526712786840633).solovay_strassen_test(1000) >= 0.75);
    assert!(ubig!(908434014620892011084840995189).solovay_strassen_test(1000) >= 0.75);
    assert!(ubig!(507898055938881191060252721391).solovay_strassen_test(1000) >= 0.75);
}

#[test]
fn huge_not_prime() {
    assert!(ubig!(424167754142049395774145538505).solovay_strassen_test(1000) < 0.75);
    assert!(ubig!(109530880042450656567646846392).solovay_strassen_test(1000) < 0.75);
    assert!(ubig!(575086839293129889716870438823).solovay_strassen_test(1000) < 0.75);
}
