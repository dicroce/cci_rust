
// This will take O(N) time. The fact that we iterate through the array twice doesn't matter.
fn foo(array: &[i32]) {
    let mut sum = 0;
    let mut product = 1;

    // 2 methods of iterating over array.

    for i in 0..array.len() {
        sum += array[i];
    }

    array.iter().for_each(|v|product *= v);

    println!("{},{}", sum, product);
}

fn main() {
    foo(&[1, 1, 2, 3, 5, 8, 13, 21]);
}
