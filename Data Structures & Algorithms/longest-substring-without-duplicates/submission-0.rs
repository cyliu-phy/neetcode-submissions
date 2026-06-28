impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut max_len = 0;
        let mut l = 0;
        let mut seen = HashSet::new();
        let bytes = s.as_bytes();

        for r in 0..bytes.len() {
            while seen.contains(&bytes[r]) {
                seen.remove(&bytes[l]);
                l += 1;
            }
            seen.insert(bytes[r]);
            max_len = max_len.max(r - l + 1);
        }
        max_len as i32
    }
}
