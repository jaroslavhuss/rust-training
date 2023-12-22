use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut hm: HashMap<i32, usize> = HashMap::with_capacity(nums.len());
    for (i, &num) in nums.iter().enumerate() {
        // Check if such a difference exists in the hash map
        match hm.get(&num) {
            // If it does, return the indices of the current number and the number with the difference
            Some(&j) => return vec![i as i32, j as i32],
            // If it doesn't, add the difference between target and the current number to the hash map
            None => {
                hm.insert(target - num, i);
            }
        }
    }
    unreachable!();
}
