impl Solution {
    pub fn encode(strs: Vec<String>) -> String {
        let mut res = String::new();

        for str in strs {
            res += &str.len().to_string();
            res += "#";
            res += &str;
        }

        res
    }

    pub fn decode(s: String) -> Vec<String> {
        let bytes = s.as_bytes();
        let mut length: usize = 0;
        let mut i: usize = 0;
        let mut res = Vec::new();

        while i < s.len() {
            if bytes[i].is_ascii_digit() {
                length = length * 10 + (bytes[i] - b'0') as usize;
                i += 1;
            } else if bytes[i] == b'#' {
                res.push(s[i + 1..i + 1 + length].to_string());
                i += 1 + length;
                length = 0;
            }
        }

        res
    }
}

