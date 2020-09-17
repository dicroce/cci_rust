
// The inner for loop has N operations and it is called N times so
// this function is O(N^2)
fn print_pairs(array: &[i32]) {
    for i in 0..array.len() {
        for j in 0..array.len() {
            println!("{},{}", array[i], array[j]);
        }
    }
}

fn main() {
    print_pairs(&[1, 1, 2, 3, 5, 8, 13, 21]);
}
