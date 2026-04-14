use std::collections::HashSet;

impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        let mut unique = HashMap::new();
        for num in nums {
            unique.entry(num)
            .and_modify(|v| *v+=1)
            .or_insert(0);
        }
        return unique.values().filter(|&&v| v > 0).count() != 0
    }
}
