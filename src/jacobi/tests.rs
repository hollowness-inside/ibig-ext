use ibig::UBig;
use std::str::FromStr;

use crate::jacobi::{Jacobi, JacobiValue};
use crate::result::Error;

macro_rules! test {
    ($($name:ident($base:expr, $modulo:expr) -> $exp:expr;)+) => {
        $(#[test]
        fn $name() {
            let base = UBig::from_str(stringify!($base)).unwrap();
            let modulo = UBig::from_str(stringify!($modulo)).unwrap();
            let result = base.jacobi(modulo);
            assert_eq!(result, $exp);
        })*
    };
}

test!(
    jacobi_15_19(15, 19) -> Ok(JacobiValue::NonResidue);
    jacobi_17_19(17, 19) -> Ok(JacobiValue::Residue);
    jacobi_5_5(5, 5) -> Ok(JacobiValue::NonCoprime);
    jacobi_1_2(1, 2) -> Err(Error::EvenModulo);
    jacobi_0_3(0, 3) -> Ok(JacobiValue::NonCoprime);
    jacobi_2_4(2, 4) -> Err(Error::EvenModulo);
    jacobi_7_3(7, 3) -> Ok(JacobiValue::Residue);
    jacobi_100_large(100, 9999999999) -> Ok(JacobiValue::Residue);
    jacobi_100_25(100, 25) -> Ok(JacobiValue::NonCoprime);
    jacobi_large(123456789, 987654321) -> Ok(JacobiValue::NonCoprime);
    jacobi_extra(1530, 99991) -> Ok(JacobiValue::NonResidue);
);
