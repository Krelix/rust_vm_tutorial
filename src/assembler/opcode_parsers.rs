use nom::{alpha1};
use nom::types::CompleteStr;

use instruction::OpCode;
use assembler::Token;

named!(pub opcode<CompleteStr, Token> ,
  do_parse!(
    value: alpha1 >>
    (Token::Op{code: OpCode::from(value)})
  )
);

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_op_load_nok() {
    let result = opcode(CompleteStr("1load1"));
    assert!(!result.is_ok());
  }
  
  #[test]
  fn test_op_load_ok() {
    let result = opcode(CompleteStr("load"));
    assert_eq!(result.is_ok(), true);
    let (rest, token) = result.unwrap();
    assert_eq!(CompleteStr(""), rest);
    assert_eq!(Token::Op{code:OpCode::LOAD}, token);

    // case insentivity check
    // FIXME OpCode::from is case sensitive... so fix this to make this test pass
    let result = opcode(CompleteStr("LOAD"));
    assert!(result.is_ok());
    let (_, token) = result.unwrap();
    assert_eq!(Token::Op{code:OpCode::IGL}, token);
  }

  #[test]
  fn test_op_illegal() {
    let result = opcode(CompleteStr("lol"));
    assert!(result.is_ok());
    let (_, token) = result.unwrap();
    assert_eq!(Token::Op{code: OpCode::IGL}, token);
  }
  
}