fn parse(inp: &str) -> Vec<Vec<i32>> {
    inp.lines().map(|line| {
        line.split_whitespace().filter_map(|num| num.parse::<i32>().ok()).collect()
    }).collect()
}

fn is_safe(rep: &[i32]) -> bool {
    if rep.len() < 2 {
        return false;
    }
    let dif : Vec<i32> = rep.windows(2).map(|pair| pair[1] - pair[0]).collect();
    dif.iter().all(|&d| d!=0 && d.abs() <= 3) && (dif.iter().all(|&d| d > 0) || dif.iter().all(|&d| d<0))
}

fn is_safe_2(rep: &[i32]) -> bool {
    is_safe(rep)
        || (0..rep.len())
            .any(|i| is_safe(&rep.iter().enumerate().filter(|&(j, _)| j != i).map(|(_, &v)| v).collect::<Vec<i32>>()))
}

pub fn solve1(inp: &str) -> usize {
    let v = parse(inp);
    let safe : Vec<_> = v.iter().enumerate().filter(|(_, rep)| is_safe(rep)).collect();
    safe.len()
}

pub fn solve2(inp: &str) -> usize {
    let v = parse(inp);
    let safe : Vec<_> = v.iter().enumerate().filter(|(_, rep)| is_safe_2(rep)).collect();
    safe.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] 
    fn test1() {
        let inp: &str =  
        "7 6 4 2 1
        1 2 7 8 9
        9 7 6 2 1
        1 3 2 4 5
        8 6 4 4 1
        1 3 6 7 9";
        assert_eq!(solve1(inp), 2);
        assert_eq!(solve2(inp), 4);
    }
}
