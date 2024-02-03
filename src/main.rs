mod quick_sort;
mod merge_sort;
use quick_sort::quick_sort;
use merge_sort::merge_sort;

fn main() {
    let input = vec![0,2,1];
    let pivot_index: usize = input.len();
    println!("Hello, world!");
    quick_sort(input, pivot_index);
    merge_sort();
}
