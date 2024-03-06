use super::FermatTest;

use ibig::ubig;

#[test]
fn prime() {
    assert!(ubig!(5).fermat_test(1000) >= 0.75);
    assert!(ubig!(13).fermat_test(1000) >= 0.75);
    assert!(ubig!(29).fermat_test(1000) >= 0.75);
    assert!(ubig!(53).fermat_test(1000) >= 0.75);
}

#[test]
fn not_prime() {
    assert!(ubig!(20).fermat_test(1000) < 0.75);
    assert!(ubig!(100).fermat_test(1000) < 0.75);
    assert!(ubig!(200).fermat_test(1000) < 0.75);
}

#[test]
fn huge_prime() {
    assert!(ubig!(659555973512315526712786840633).fermat_test(1000) >= 0.75);
    assert!(ubig!(908434014620892011084840995189).fermat_test(1000) >= 0.75);
    assert!(ubig!(507898055938881191060252721391).fermat_test(1000) >= 0.75);
}

#[test]
fn huge_not_prime() {
    assert!(ubig!(424167754142049395774145538505).fermat_test(1000) < 0.75);
    assert!(ubig!(109530880042450656567646846392).fermat_test(1000) < 0.75);
    assert!(ubig!(575086839293129889716870438823).fermat_test(1000) < 0.75);
}
