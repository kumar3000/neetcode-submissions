use std::collections::HashSet;

impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        let mut unique = HashSet::new();

        for n in nums {
            if unique.contains(&n) {
                return true
            } else {
                unique.insert(n);
            }
        }
        return false
    }
}
