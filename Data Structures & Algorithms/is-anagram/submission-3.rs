impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false
        }

        let mut map = HashMap::new();

        for c in s.chars() {
            *map.entry(c).or_insert(0) += 1;
        }

        for c in t.chars() {
            let count = map.entry(c).or_insert(0);
            if *count == 0 {
                return false;
            }
            *count -= 1;
        }

        map.values().all(|&count| count == 0)
    }
}
