const MAP: [(&str, u32); 9] = [
  ("one", 1),
  ("two", 2),
  ("three", 3),
  ("four", 4),
  ("five", 5),
  ("six", 6),
  ("seven", 7),
  ("eight", 8),
  ("nine", 9)
];

fn get_first_parsed_number(line: &str) -> Option<u32> {
  let position = line.split("")
    .position(|s| s.parse::<u32>().is_ok())?;

  Some(line.split("").collect::<Vec<&str>>()[position].parse::<u32>().unwrap())
}

fn get_first_item(line: &str) -> Option<u32> {
  // let result = MAP.iter().find_map(|item| {
  //   let (key, value) = item;

  //   let pos = line.find(|s|) 
  // });

  get_first_parsed_number(line)
}

fn get_last_item(line: &str) -> Option<u32> {
  let line = line.chars().rev().collect::<String>();

  get_first_parsed_number(&line)
}

fn get_parseable_items(line: &str) -> Option<(u32, u32)> {
  let first = get_first_item(line)?;
  let last = get_last_item(line)?;

  Some((first, last))
}

fn handle_input(input: &str) -> u32 {
  let inputs = input.split("\n");

  let sum = inputs
    .into_iter()
    .map(get_parseable_items)
    .map(|values| {
      match values {
        Some((first, last)) => format!("{first}{last}").parse::<u32>().unwrap(),
        _ => panic!("Unexpected non Some(first, last)")
      }
    })
    .reduce(|acc, cur| acc + cur);

  match sum {
    Some(result) => result,
    None => panic!("Unexpected sum None")
  }
}

fn main() {
  let input = include_str!("./input.txt");
  let result = handle_input(input);

  dbg!(result);
}

#[cfg(test)]
mod tests {
  use super::*;

  macro_rules! test_case {
    ($name:ident, $input:expr, $expected:expr) => {
      #[test]
      fn $name() {
        let input = $input;
        let expected = $expected;
        let actual = handle_input(input);
        assert_eq!(actual, expected);
      }
    };
  }
  
  test_case!(two1nine, "two1nine", 29);

  test_case!(eightwothree, "eightwothree", 83);

  test_case!(abcone2threexyz, "abcone2threexyz", 13);
  
  test_case!(oneabc2, "1abc2", 12);

  test_case!(pqr3stu8vwx, "pqr3stu8vwx", 38);

  test_case!(a1b2c3d4e5f, "a1b2c3d4e5f", 15);

  test_case!(treb7uchet, "treb7uchet", 77);

  test_case!(
    multiple_lines1,
    r"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet",
    142);

  test_case!(xtwone3four, "xtwone3four", 24);

  test_case!(r4nineeightseven2, "4nineeightseven2", 42);

  test_case!(zoneight234, "zoneight234", 14);

  test_case!(r7pqrstsixteen, "7pqrstsixteen", 76);

  test_case!(
    multiple_lines2,
    r"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen",
    142);
}
