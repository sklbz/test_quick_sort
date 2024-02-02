mod quick_sort;
mod merge_sort;
use quick_sort::quick_sort;
use merge_sort::merge_sort;

fn main() {
    let input = vec![0,2,1];
    println!("Hello, world!");
    quick_sort(input);
    merge_sort();
}
