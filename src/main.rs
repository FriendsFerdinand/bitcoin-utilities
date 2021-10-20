mod ecc;
use ecc::FieldElement;
mod errors;
use ibig::ubig;

fn main() {
    println!("Hello, world!");
    // ecc::FieldElement::new(ubig!(4), ubig!(31)).unwrap();
    let a = ecc::FieldElementOps.new(ubig!(4), ubig!(31)).unwrap();
}
