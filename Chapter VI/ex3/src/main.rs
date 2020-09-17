
// Note: inner loop starting range is current value of i+1. Despite this
// it will still take O(N^2) time.
fn print_unordered_pairs(array: &[i32]) {
    for i in 0..array.len() {
        for j in i+1..array.len() {
            println!("{}, {}", array[i], array[j]);
        }
    }
}

fn main() {
    print_unordered_pairs(&[1, 1, 2, 3, 5, 8, 13, 21]);
}
