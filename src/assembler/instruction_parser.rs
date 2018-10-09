use std;
use nom::types::CompleteStr;
use assembler::Token;
use assembler::opcode_parsers::*;
use assembler::register_parsers::register;
use assembler::operand_parsers::int_operand;

use instruction::OpCode;

#[derive(Debug, PartialEq)]
pub struct AssemblerInstruction {
  opcode: Token,
  operand1: Option<Token>,
  operand2: Option<Token>,
  operand3: Option<Token>
}

impl AssemblerInstruction {

  pub fn to_bytes(&self) -> Vec<u8> {
    let mut result = vec![];
    match self.opcode {
      Token::Op{code} => result.push(code as u8),
      _ => {
        println!("Unknown OpCode found {:?}", self.opcode);
        std::process::exit(1);
      }
    }

    for operand in &[&self.operand1, &self.operand2, &self.operand3] {
      match operand {
        Some(t) => AssemblerInstruction::extract_operand(t, &mut result),
        None => {}
      }
    }

    result
  }

  fn extract_operand(token: &Token, results: &mut Vec<u8>) {
    match token {
      Token::Register { reg_num } => {
        results.push(*reg_num);
      },
      Token::IntegerOperand { value } => {
        let value_u16 = *value as u16;
        let last_4_bytes = value_u16 as u8;
        let first_4_bytes = (value_u16 >> 8) as u8;
        results.push(first_4_bytes);
        results.push(last_4_bytes);
      }
      _ => {
        println!("Unknown operand found {:?}", token);
        std::process::exit(1);
      }
    }
  }
}

named!{pub instruction<CompleteStr, AssemblerInstruction>,
  do_parse!(
    code: op_load >>
    reg: register >>
    operand: int_operand >> (
      AssemblerInstruction{
        opcode: code,
        operand1: Some(reg),
        operand2: Some(operand),
        operand3: None
      }
    )
  )
}

#[cfg(test)]
mod tests {
  use super::*;
  
  #[test]
  fn test_parse_instruction_ok() {
    let result = instruction(CompleteStr("load $1 #200"));
    assert!(result.is_ok());
    let (_, instruction) = result.unwrap();
    assert_eq!(Token::Op{code: OpCode::LOAD}, instruction.opcode);
    assert_eq!(Some(Token::Register{reg_num: 1}), instruction.operand1);
    assert_eq!(Some(Token::IntegerOperand{value: 200}), instruction.operand2);
    assert_eq!(None, instruction.operand3);
  }

  #[test]
  fn test_to_bytes() {
      let result = instruction(CompleteStr("load $1 #500"));
      assert!(result.is_ok());
      let (_, instr) = result.unwrap();
      let bytes = instr.to_bytes();
      assert_eq!(4, bytes.len());
      assert_eq!(0, bytes[0]);
      assert_eq!(1, bytes[1]);
      // 256 for 1st byte
      assert_eq!(1, bytes[2]);
      assert_eq!(244, bytes[3]);
  }
}

#[test]
fn test_parse_instruction_form_one() {
    let result = instruction(CompleteStr("load $0 #100\n"));
    assert_eq!(
        result,
        Ok((
            CompleteStr(""),
            AssemblerInstruction {
                opcode: Token::Op{ code: OpCode::LOAD },
                operand1: Some(Token::Register { reg_num: 0 }),
                operand2: Some(Token::IntegerOperand { value: 100 }),
                operand3: None
            }
        ))
    );
}