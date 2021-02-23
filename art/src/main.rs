extern crate art;

use art::{PrimaryColor, mix};


fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);
}