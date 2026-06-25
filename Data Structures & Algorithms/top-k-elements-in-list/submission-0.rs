impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut freq = HashMap::new();
        for num in nums {
            *freq.entry(num).or_insert(0) += 1;
        }

        let mut values: Vec<_> = freq.values().copied().collect();
        values.sort_unstable_by(|a, b| b.cmp(a));

        let topk: HashSet<i32> = values.into_iter().take(k as usize).collect();
        let mut res: Vec<i32> = Vec::new();

        for (k, v) in freq {
            if topk.contains(&v) {
                res.push(k);
            }
        }
        res
    }
}
