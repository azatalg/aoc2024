use std::collections::HashMap;
fn format_vec(input: &str) -> (Vec<i32>, Vec<i32>) {
    input.lines().fold((Vec::new(), Vec::new()), |(mut left, mut right), line| {
        let nums: Vec<i32> = line
            .split_whitespace()
            .filter_map(|num| num.parse::<i32>().ok())
            .collect();
        left.push(nums[0]);
        right.push(nums[1]);
        (left, right)
    })
}

pub fn solve1(inp: &str) -> i32 {
    let (mut a, mut b) = format_vec(inp);
    a.sort();
    b.sort();
    a.iter().zip(b.iter()).fold(0, |acc, (l, r)| acc+(l-r).abs())
}

pub fn solve2(inp: &str) -> i32 {
    let (a, b) = format_vec(inp);
      
    let frequency = b.iter().fold(HashMap::new(), |mut m, &num| {
        *m.entry(num).or_insert(0) += 1;
        m
    });

    a.iter().fold(0, |acc, &num| {
        acc+num*frequency.get(&num).unwrap_or(&0)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] 
    fn test1() {
        let inp: &str =  
        "3   4
         4   3
         2   5
         1   3
         3   9
         3   3";
        assert_eq!(solve1(inp), 11);
        assert_eq!(solve2(inp), 31);
    }
}
