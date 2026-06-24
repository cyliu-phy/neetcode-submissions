impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut s1 = [0; 26];
        let mut s2 = [0; 26];
        for c in s.chars() {
            s1[c as usize - 'a' as usize] += 1;
        }
        for c in t.chars() {
            s2[c as usize - 'a' as usize] += 1;
        }
        s1 == s2
    }
}
