impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        let mut uniq_nums = HashSet::new();
        for &num in &nums {
            if uniq_nums.contains(&num) {
                return true;
            } else {
                uniq_nums.insert(num);
            }
        }
        false
    }
}
