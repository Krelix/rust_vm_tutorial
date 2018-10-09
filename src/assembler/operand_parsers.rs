use assembler::Token;
use nom::digit;
use nom::types::CompleteStr;

// FIXME: negative integers ?
named!{pub int_operand<CompleteStr,Token>,
  ws!(
    do_parse!(
      tag!("#") >>

      operand: digit >> (
        Token::IntegerOperand{ value : operand.parse::<i32>().unwrap()}
      )
    )
  )
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_parse_int_operand_nok() {
    let result = int_operand(CompleteStr("100"));
    assert!(!result.is_ok());

		let result = int_operand(CompleteStr("#abc"));
    assert!(!result.is_ok());
  }
  
  #[test]
  fn test_parse_int_operand_ok() {
    let result = int_operand(CompleteStr("#200"));
    assert!(result.is_ok());
    let (rest, token) = result.unwrap();
    assert_eq!(CompleteStr(""), rest);
    assert_eq!(Token::IntegerOperand{value: 200}, token);
  }
  
}