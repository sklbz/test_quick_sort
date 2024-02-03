pub fn quick_sort(input: Vec<u32>, pivot_index: usize) -> Vec<u32>{
    let pivot_value = input[pivot_index];
    let mut nums = input.clone();
    let mut i: isize = -1;
    for j in 0..=pivot_index {
        if nums[j] >= pivot_value {
            i += 1;
        } else {
            let temp = nums[j];
            nums[j] = nums[i as usize];
            nums[i as usize] = temp;
        }
    }
    return nums;
}