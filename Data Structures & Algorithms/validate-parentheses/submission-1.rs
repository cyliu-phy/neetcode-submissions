impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();
        let lefts = HashMap::from([('(', 1), ('[', 2), ('{', 3)]);

        let rights = HashMap::from([(')', 1), (']', 2), ('}', 3)]);

        for i in s.chars() {
            if lefts.contains_key(&i) {
                stack.push(i);
            } else {
                if let Some(left) = stack.pop() {
                    if lefts[&left] != rights[&i] {
                        return false;
                    }
                } else {
                    return false;
                }
            }
        }
        stack.is_empty()
    }
}
