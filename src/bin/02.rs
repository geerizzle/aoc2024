advent_of_code::solution!(2);

pub fn is_safe(report: &Vec<i32>) -> bool {
    report
        .windows(2)
        .all(|x| (x[0] - x[1]).abs() <= 3 && x[0] < x[1])
        || report
            .windows(2)
            .all(|x| (x[0] - x[1]).abs() <= 3 && x[0] > x[1])
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut safe: u32 = 0;
    for line in input.lines() {
        let line: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        if is_safe(&line) {
            safe += 1
        }
    }
    Some(safe)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut safe: u32 = 0;
    for line in input.lines() {
        let line: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        if is_safe(&line) {
            safe += 1;
            continue;
        }

        for i in 0..line.len() {
            let mut new_report = line.clone();
            new_report.remove(i);
            if is_safe(&new_report) {
                safe += 1;
                break;
            }
        }
    }
    Some(safe)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
