impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let mut i = 0;
        let mut j = s.len() - 1;

        let bytes = s.as_bytes();
        while i < j {
            if !bytes[i].is_ascii_alphanumeric() {
                i += 1;
                continue;
            }
            if !bytes[j].is_ascii_alphanumeric() {
                j -= 1;
                continue;
            }
            if bytes[i].to_ascii_lowercase() != bytes[j].to_ascii_lowercase() {
                return false;
            } else {
                i += 1;
                j -= 1;
            }
        }

        true
    }
}

