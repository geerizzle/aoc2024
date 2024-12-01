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
    list1.sort_unstable();
    list2.sort_unstable();
    let sum = list1
        .iter()
        .zip(list2.iter())
        .map(|(a, b)| (a - b).abs())
        .sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<i32> {
    let (list1, list2) = parse_input(input);
    let mut map: HashMap<&i32, i32> = HashMap::new();
    for item in &list2[..list2.len() - 1] {
        *map.entry(item).or_insert(0) += 1;
    }
    let sum: i32 = list1.iter().map(|i| i * map.get(i).unwrap_or(&0)).sum();
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
