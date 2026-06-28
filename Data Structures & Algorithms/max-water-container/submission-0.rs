impl Solution {
    pub fn max_area(heights: Vec<i32>) -> i32 {
        let mut start = 0usize;
        let mut end = heights.len() - 1;
        let mut max = 0;

        while start < end {
            let area = heights[end].min(heights[start]) * (end - start) as i32;
            max = max.max(area);
            if heights[start] > heights[end] {
                end -= 1;
            } else {
                start += 1;
            }
        }
        max
    }
}
