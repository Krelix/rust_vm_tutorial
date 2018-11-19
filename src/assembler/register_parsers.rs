use nom::digit;
use nom::types::CompleteStr;

use assembler::Token;

// parse $10 by removing whitspaces around it and parsing the number into the Token instance
named!{pub register<CompleteStr, Token>,
  ws!(
    do_parse!(
      tag!("$") >>
      reg_num: digit >> (
        Token::Register {
          reg_num : reg_num.parse::<u8>().unwrap()
        }
      )
    )
  )
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_register_nok() {
    let result = register(CompleteStr("20"));
    assert!(!result.is_ok());
    let result = register(CompleteStr("$a"));
    assert!(!result.is_ok());
  }

  #[test]
  fn test_register_ok() {
    let result = register(CompleteStr("$20"));
    assert!(result.is_ok());
    let (rest, token) = result.unwrap();
    assert_eq!(CompleteStr(""), rest);
    assert_eq!(Token::Register { reg_num: 20 }, token);
  }

}
