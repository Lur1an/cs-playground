use std::slice::Iter;

pub fn max_area(height: Vec<i32>) -> i32 {
    let mut i = 0;
    let mut j = height.len() - 1;
    // assume l >= k
    let calc_area = |k: usize, l: usize| std::cmp::min(height[k], height[l]) * (l - k) as i32;

    let mut area_guess = 0;
    while i != j {
        area_guess = std::cmp::max(area_guess, calc_area(i, j));
        if height[i] > height[j] {
            j -= 1;
        } else {
            i += 1;
        }
    }
    return area_guess;
}

#[cfg(test)]
mod test {}
