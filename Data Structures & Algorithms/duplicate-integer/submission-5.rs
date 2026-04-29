impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        let mut unique = HashSet::new();
        let mut i = 0;
        while i < nums.len() {
            if unique.contains(&nums[i]) {
                return true;
            } else {
                unique.insert(nums[i]);
                i += 1;
            }
        }
        return false;
    }
}
