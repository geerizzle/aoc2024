use std::collections::HashMap;

advent_of_code::solution!(1);

fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut list1: Vec<i32> = vec![];
    let mut list2: Vec<i32> = vec![];
    for line in input.split("\n") {
        let (first, second) = line.split_once(" ").unwrap_or(("99999", "99999"));
        let first = first.trim().parse::<i32>().unwrap();
        let second = second.trim().parse::<i32>().unwrap();
        list1.push(first);
        list2.push(second);
    }
    (list1, list2)
}

pub fn part_one(input: &str) -> Option<i32> {
    let (mut list1, mut list2) = parse_input(input);
    list1.sort();
    list2.sort();
    let sum = list1.iter().zip(list2.iter()).fold(0, |acc, (a, b)| {
        if a == b {
            return acc;
        }
        let dist: i32 = (a - b).abs();
        acc + dist
    });
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (list1, list2) = parse_input(input);
    let mut map: HashMap<&i32, u32> = HashMap::new();
    for item in &list2[..list2.len() - 1] {
        *map.entry(item).or_insert(0) += 1;
    }
    let sum: u32 = list1.iter().fold(0, |acc, item| {
        let count: &u32 = map.get(item).unwrap_or(&0);
        acc + (count * (*item as u32))
    });
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
