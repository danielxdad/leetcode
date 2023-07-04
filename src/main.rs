#[allow(dead_code)]
mod longest_palindrome;
mod elimination_game;
mod make_array_strictly_increasing;

fn main() {
    println!("Run: cargo test --tests");

    println!("{}", make_array_strictly_increasing::make_array_increasing_v2([0,11,6,1,4,3].to_vec(), [5,4,11,10,1,0].to_vec()));
}
