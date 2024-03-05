use crate::PowMod;
use ibig::{IBig, UBig};
use std::str::FromStr;

macro_rules! test {
        (UBIG $($name:ident($base:literal ^ $exponent:literal % $divisor:literal = $expected:literal),)*) => {
            $(
                #[test]
                fn $name() {
                    let base = UBig::from_str(stringify!($base)).unwrap();
                    let exponent = UBig::from_str(stringify!($exponent)).unwrap();
                    let divisor = UBig::from_str(stringify!($divisor)).unwrap();

                    let remainder = base.powmod(exponent, &divisor);
                    let expected = UBig::from_str(stringify!($expected)).unwrap();

                    assert_eq!(remainder, expected);
                }
            )*
        };

        (IBIG $($name:ident($base:literal ^ $exponent:literal % $divisor:literal = $expected:literal),)*) => {
            $(
                #[test]
                fn $name() {
                    let base = IBig::from_str(stringify!($base)).unwrap();
                    let exponent = UBig::from_str(stringify!($exponent)).unwrap();
                    let divisor = IBig::from_str(stringify!($divisor)).unwrap();

                    let remainder = base.powmod(exponent, &divisor);
                    let expected = IBig::from_str(stringify!($expected)).unwrap();

                    assert_eq!(remainder, expected);
                }
            )*
        };
    }

test!(UBIG
    test_u1(23 ^ 2342 % 13 = 9),
    test_u2(396 ^ 3064 % 31 = 14),
    test_u3(487 ^ 9045 % 43 = 41),
    test_u4(960 ^ 4375 % 73 = 5),
);

test!(IBIG
    test_i1(-960 ^ 4375 % 73 = 68),
    test_i2(-769856132 ^ 56445 % 123 = 79),
);
