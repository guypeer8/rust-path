mod ownership;
mod borrow;

fn main() {
    let values_ownership = vec![1, 2, 3, 4, 5];
    let values_borrow = vec![1, 2, 3, 4, 5];

    println!("ownership sum is: {}", ownership::sum(values_ownership));
    println!("failing because of ownership: {}", values_ownership.len());

    println!("borrow sum is: {}", borrow::sum(&values_borrow));
    println!("succeeding because of borrow: {}", values_borrow.len());
}
