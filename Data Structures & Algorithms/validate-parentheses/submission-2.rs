impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut st = Vec::new();
        for char in s.chars() {
            match char {
                '(' => st.push(')'),
                '[' => st.push(']'),
                '{' => st.push('}'),
                ')' | ']' | '}' => {
                    if let Some(val) = st.pop() {
                        if val != char {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
                _ => unreachable!(),
            }
        }
        st.is_empty()
    }
}
