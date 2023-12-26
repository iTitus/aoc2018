use aoc_runner_derive::{aoc, aoc_generator};
use rustc_hash::FxHashSet;

use crate::common::parse_lines;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i64> {
    parse_lines(input).unwrap()
}

fn simulate(changes: &[i64]) -> i64 {
    changes.iter().sum()
}

fn check_for_double_visit(changes: &[i64]) -> i64 {
    let mut visited = FxHashSet::default();
    let mut current = 0;
    for i in changes.iter().cycle() {
        if !visited.insert(current) {
            return current;
        }

        current += i;
    }

    unreachable!();
}

#[aoc(day1, part1)]
pub fn part1(input: &[i64]) -> i64 {
    simulate(input)
}

#[aoc(day1, part2)]
pub fn part2(input: &[i64]) -> i64 {
    check_for_double_visit(input)
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_part1_1() {
        const INPUT: &str = r#"+1
-2
+3
+1"#;
        assert_eq!(part1(&input_generator(INPUT)), 3);
    }

    #[test]
    fn test_part1_2() {
        const INPUT: &str = r#"+1
+1
+1"#;
        assert_eq!(part1(&input_generator(INPUT)), 3);
    }

    #[test]
    fn test_part1_3() {
        const INPUT: &str = r#"+1
+1
-2"#;
        assert_eq!(part1(&input_generator(INPUT)), 0);
    }

    #[test]
    fn test_part1_4() {
        const INPUT: &str = r#"-1
-2
-3"#;
        assert_eq!(part1(&input_generator(INPUT)), -6);
    }

    #[test]
    fn test_part2_1() {
        const INPUT: &str = r#"+1
-1"#;
        assert_eq!(part2(&input_generator(INPUT)), 0);
    }

    #[test]
    fn test_part2_2() {
        const INPUT: &str = r#"+3
+3
+4
-2
-4"#;
        assert_eq!(part2(&input_generator(INPUT)), 10);
    }

    #[test]
    fn test_part2_3() {
        const INPUT: &str = r#"-6
+3
+8
+5
-6"#;
        assert_eq!(part2(&input_generator(INPUT)), 5);
    }

    #[test]
    fn test_part2_4() {
        const INPUT: &str = r#"+7
+7
-2
-7
-4"#;
        assert_eq!(part2(&input_generator(INPUT)), 14);
    }
}
