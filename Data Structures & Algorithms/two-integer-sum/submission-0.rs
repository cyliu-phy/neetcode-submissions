impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut uniq_map: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut res = Vec::new();

        for (i, &num) in nums.iter().enumerate() {
            uniq_map.entry(num).or_insert_with(Vec::new).push(i as i32);
        }

        for (i, &num) in nums.iter().enumerate() {
            if uniq_map.contains_key(&(target - num)) && target - num != num {
                res.push(i as i32);
                res.push(uniq_map[&(target - num)][0]);
                break;
            } else if target - num == num && uniq_map[&num].len() >= 2 {
                return uniq_map[&num][..2].to_vec();
            }
        }
        res
    }
}
