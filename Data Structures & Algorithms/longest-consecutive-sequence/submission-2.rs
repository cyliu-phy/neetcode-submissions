impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        
        let uniq_items: HashSet<&i32> = nums.iter().collect();
        let mut res = 0;

        for &num in &nums {
            if !uniq_items.contains(&(num - 1)) {
                let mut length = 1;
                while uniq_items.contains(&(num + length)) {
                    length += 1;
                }
                res = res.max(length);
            }
        }

        res
    }
}
