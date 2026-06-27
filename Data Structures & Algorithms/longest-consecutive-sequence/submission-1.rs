impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut nums = nums.clone();
        nums.sort_unstable();

        let mut curr = nums[0];
        let mut longest = 1;
        let mut streak = 1;

        for &num in &nums[1..] {
            if num == curr {
                continue;
            } else if num == curr + 1 {
                streak += 1;
            } else {
                longest = longest.max(streak);
                streak = 1;
            }
            curr = num;
        }
        longest = longest.max(streak);

        longest
    }
}
