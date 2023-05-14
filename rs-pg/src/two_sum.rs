use std::collections::HashMap;

pub fn two_sum_hash(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut index_map: HashMap<i32, usize> = HashMap::new();
    for i in 0..nums.len() {
        let x = nums[i];
        let y = target - x;
        if let Some(j) = index_map.get(&y) {
            return vec![i as i32, *j as i32];
        }
        index_map.insert(x, i);
    }
    panic!();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_two_sum() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let result = two_sum_hash(nums, target);
        assert_eq!(result, vec![0, 1]);
    }
}
