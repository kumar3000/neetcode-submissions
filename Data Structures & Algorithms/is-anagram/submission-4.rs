impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut s_vec: Vec<char> = s.chars().collect();
        let mut t_vec: Vec<char> = t.chars().collect();

        s_vec.sort();
        t_vec.sort();
        let mut s_sorted: String = s_vec.into_iter().collect();
        let mut t_sorted: String = t_vec.into_iter().collect();

        if s_sorted != t_sorted {
            return false;
        }

        return true;
    }
}
