mod input;
mod test;

use input::input;
use test::arr;

fn main() {
    let (tes, num, tmp) = input();
    println!("The original array is {:?}", &tes[0..num]);
    let (feed, ind) = arr(tes, tmp, num);
    println!("The new array is {:?}", &feed[0..ind]);
}
