use parse_display::{Display as PDisplay, FromStr as PFromStr};
use std::error::Error;

fn parse_password_rules(input: &str) -> Result<Vec<PasswordRule>, impl Error> {
  input
    .lines()
    .map(|line| line.parse::<PasswordRule>())
    .collect()
}

#[derive(PDisplay, PFromStr, Debug)]
#[display("{min}-{max} {letter}: {password}")]
struct PasswordRule {
  min: usize,
  max: usize,
  letter: char,
  password: String,
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &str) -> usize {
  parse_password_rules(input)
    .unwrap()
    .into_iter()
    .map(|rule| (rule.min, rule.max, rule.password.chars().filter(|&ch| rule.letter == ch).count()))
    .filter(|(min, max, count)| min <= count && count <= max)
    .count()
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &str) -> usize {
  parse_password_rules(input)
    .unwrap()
    .into_iter()
    .filter(|rule| {
      let mut pw_chars = rule.password.chars();
      let min = pw_chars.nth(rule.min - 1);
      let max = pw_chars.nth(rule.max - rule.min - 1);
      (max == Some(rule.letter)) ^ (min == Some(rule.letter))
    })
    .count()
}
