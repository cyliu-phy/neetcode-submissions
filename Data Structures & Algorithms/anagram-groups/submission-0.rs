impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut seen: HashMap<Vec<char>, Vec<String>> = HashMap::new();
        for s in strs {
            let mut key: Vec<_> = s.chars().collect();
            key.sort_unstable();
            seen.entry(key).or_insert_with(Vec::new).push(s);
        }
        seen.into_values().collect()
    }
}
