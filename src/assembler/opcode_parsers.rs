use nom::types::CompleteStr;

use instruction::OpCode;
use assembler::Token;

named!{pub op_load<CompleteStr, Token>, 
  do_parse!(
    tag_no_case!("load") >> 
    (Token::Op{code: OpCode::LOAD})
  )
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_op_load_nok() {
    let result = op_load(CompleteStr("none"));
    assert!(!result.is_ok());
  }
  
  #[test]
  fn test_op_load_ok() {
    let result = op_load(CompleteStr("load"));
    assert_eq!(result.is_ok(), true);
    let (rest, token) = result.unwrap();
    assert_eq!(CompleteStr(""), rest);
    assert_eq!(Token::Op{code:OpCode::LOAD}, token);

    // case insentivity check
    let result = op_load(CompleteStr("LOAD"));
    assert!(result.is_ok());
    let (rest, token) = result.unwrap();
    assert_eq!(CompleteStr(""), rest);
    assert_eq!(Token::Op{code:OpCode::LOAD}, token);
  }
  
}