fn get_parseable_position(line: &str) -> Option<usize> {
  line.split("")
    .position(|s| s.parse::<u32>().is_ok())
}

fn get_parseable_items(line: &str) -> Option<(u32, u32)> {
  let first_position = get_parseable_position(line)?;
  let reversed_line = line.chars().rev().collect::<String>();
  let last_position = get_parseable_position(&reversed_line)?;

  let reversed_line = reversed_line.split("").collect::<Vec<&str>>();
  let line = line.split("").collect::<Vec<&str>>();

  Some((line[first_position].parse::<u32>().unwrap(), reversed_line[last_position].parse::<u32>().unwrap()))
}

fn handle_input(input: &str) -> Option<u32> {
  let inputs = input.split("\n");

  let sum = inputs
    .into_iter()
    .map(get_parseable_items)
    .map(|values| {
      match values {
        Some((first, last)) => format!("{first}{last}").parse::<u32>().unwrap(),
        _ => {
          println!("Unexpected non Some(first, last)");
          0
        }
      }
    })
    .reduce(|acc, cur| acc + cur)?;

  return Some(sum);
}

fn main() {
  let input = include_str!("./input.txt");
  let result = handle_input(input);

  match result {
    Some(output) => dbg!(output),
    None => {
      println!("Unexpected none");
      0
    }
  };
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
        assert_eq!(actual, Some(expected));
      }
    };
  }
  
  test_case!(oneabc2, "1abc2", 12);

  test_case!(pqr3stu8vwx, "pqr3stu8vwx", 38);

  test_case!(a1b2c3d4e5f, "a1b2c3d4e5f", 15);

  test_case!(treb7uchet, "treb7uchet", 77);

  test_case!(
    multiple_lines,
    r"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet",
    142);
  
  test_case!(input, include_str!("./input.txt"), 54634);
}
