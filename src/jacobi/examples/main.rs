use ibig::ubig;
use jacobi::Jacobi;

fn main() {
    let nominator = ubig!(100);
    let denominator = ubig!(9999999999);
    let j = nominator.jacobi(denominator.clone()).unwrap();

    println!("({nominator}|{denominator}) = {:?}", j as i8)
}
