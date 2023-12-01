fn handle_input(input: &str) -> Option<u32> {
  todo!()
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
}
