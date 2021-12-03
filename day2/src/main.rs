use std::fs;
use regex::Regex;

fn load_data(file: &str) -> Vec<Vec<u32>> {
    let re = Regex::new(r"(\d+)").unwrap();
    fs::read_to_string(file)
        .unwrap()
        .lines()
        .map(|line| {
            re
                .captures_iter(line)
                .map(|x| x.get(1).unwrap().as_str().parse().unwrap())
                .collect()
        })
        .collect()
}

fn max<T : Ord + Copy>(vec: &Vec<T>) -> T {
    vec.iter().fold(vec[0], |acc, elem| acc.max(*elem))
}

fn min<T : Ord + Copy>(vec: &Vec<T>) -> T {
    vec.iter().fold(vec[0], |acc, elem| acc.min(*elem))
}

fn solution1(data: &Vec<Vec<u32>>) -> u32 {
    data
        .iter()
        .map(|row| max(row) - min(row))
        .sum()
}

fn solution2(data: &Vec<Vec<u32>>) -> u32 {
    data
        .iter()
        .filter_map(|row| {
            for x in row {
                for y in row {
                    if x != y && x > y {
                        if x % y == 0 {
                            return Some(x / y);
                        }
                    }
                }
            }
            None
        })
        .sum()
}

fn main() {
    let data = load_data("input");
    let result1 = solution1(&data);
    println!("Solution 1: {}", result1);
    let result2 = solution2(&data);
    println!("Solution 2: {}", result2);
}

#[cfg(test)]
mod tests {
    #[test]
    fn sample1() {
        let data = crate::load_data("sample1");
        let result = crate::solution1(&data);
        assert_eq!(18, result);
    }

    #[test]
    fn sample2() {
        let data = crate::load_data("sample2");
        let result = crate::solution2(&data);
        assert_eq!(9, result);
    }

    #[test]
    fn solution1() {
        let data = crate::load_data("input");
        let result = crate::solution1(&data);
        assert_eq!(51833, result);
    }

    #[test]
    fn solution2() {
        let data = crate::load_data("input");
        let result = crate::solution2(&data);
        assert_eq!(288, result);
    }
}
