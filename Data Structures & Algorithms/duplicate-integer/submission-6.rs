impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        let mut unique = HashSet::new();
        for num in nums.iter() {
            if unique.contains(&num) {
                return true;
            } else {
                unique.insert(num);
            }
        }
        return false;
    }
}
