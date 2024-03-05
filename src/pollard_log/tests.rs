use ibig::{IBig, UBig};
use std::str::FromStr;

use crate::discrete_log;

macro_rules! test {
    ($($name:ident: $base:literal ^ $log:literal = $target:literal % $div:literal;)*) => {
        $(#[test]
        fn $name() {
            let base = IBig::from_str(stringify!($base)).unwrap();
            let target = IBig::from_str(stringify!($target)).unwrap();
            let div = IBig::from_str(stringify!($div)).unwrap();

            let log = UBig::from_str(stringify!($log)).unwrap();

            match discrete_log(&base, &target, &div) {
                Some(res) => assert_eq!(log, res),
                None => panic!("Expected some value"),
            }
        })*
    };
}

test!(
    test_0: 4184873 ^ 2817008 = 1882449 % 4607807;
    test_1: 1788808 ^ 2975406 = 1945966 % 4608223;
    test_2: 2477711 ^ 513010 = 2307803 % 4607423;
    test_3: 1453011 ^ 1330771 = 1854580 % 4607411;
    test_4: 1787147 ^ 2778999 = 2924233 % 4607819;
    test_5: 1493107 ^ 1460886 = 1699241 % 4607387;
    test_6: 3066987 ^ 3931646 = 805198 % 4607431;
    test_7: 4265505 ^ 3657320 = 199782 % 4607807;
    test_8: 226306 ^ 2564662 = 4147786 % 4607963;
    test_9: 2670375 ^ 3048814 = 4477985 % 4607719;
    test_a: 23 ^ 1216 = 53 % 1223;
);
