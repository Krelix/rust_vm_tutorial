use assembler::opcode_parsers::*;
use assembler::operand_parsers::operand;
use assembler::directive_parsers::directive;
use assembler::label_parsers::label_declaration;
use assembler::Token;

use nom::types::CompleteStr;
use std;

#[derive(Debug, PartialEq)]
pub struct AssemblerInstruction {
  pub opcode: Option<Token>,
  pub label: Option<Token>,
  pub directive: Option<Token>,
  pub operand1: Option<Token>,
  pub operand2: Option<Token>,
  pub operand3: Option<Token>,
}

impl AssemblerInstruction {
  pub fn to_bytes(&self) -> Vec<u8> {
    let mut result = vec![];
    match self.opcode {
      Some(Token::Op{code}) => result.push(code as u8),
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
    // pad to full 32 bit length
    while result.len() < 4 {
      result.push(0);
    }

    result
  }

  fn extract_operand(token: &Token, results: &mut Vec<u8>) {
    match token {
      Token::Register { reg_num } => {
        results.push(*reg_num);
      }
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

named!(instruction_combined<CompleteStr, AssemblerInstruction>,
  do_parse!(
    l: opt!(label_declaration) >>
    o: opcode >>
    o1: opt!(operand) >>
    o2: opt!(operand) >>
    o3: opt!(operand) >>
    (
      AssemblerInstruction{
        opcode: Some(o),
        label: l,
        directive: None,
        operand1: o1,
        operand2: o2,
        operand3: o3
      }
    )
  )
);

named!(pub instruction<CompleteStr, AssemblerInstruction>,
  do_parse!(
    ins: alt!(instruction_combined | directive) >>
    (
      ins
    )
  )
);

#[cfg(test)]
mod tests {
  use super::*;
  use instruction::OpCode;

  #[test]
  fn test_parse_instruction_ok() {
    let result = instruction(CompleteStr("load $1 #200"));
    assert!(result.is_ok());
    let (_, instruction) = result.unwrap();
    assert_eq!(Token::Op { code: OpCode::LOAD }, instruction.opcode.unwrap());
    assert_eq!(Some(Token::Register { reg_num: 1 }), instruction.operand1);
    assert_eq!(
      Some(Token::IntegerOperand { value: 200 }),
      instruction.operand2
    );
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

  #[test]
  fn test_parse_instruction_form_one() {
    let result = instruction(CompleteStr("load $0 #100\n"));
    assert_eq!(
      result,
      Ok((
        CompleteStr(""),
        AssemblerInstruction {
          label: None,
          directive: None,
          opcode: Some(Token::Op { code: OpCode::LOAD }),
          operand1: Some(Token::Register { reg_num: 0 }),
          operand2: Some(Token::IntegerOperand { value: 100 }),
          operand3: None
        }
      ))
    );
  }

  #[test]
  fn test_parse_add() {
      let result = instruction(CompleteStr("add $0 $1 $2"));
      assert_eq!(
        Ok((
          CompleteStr(""),
          AssemblerInstruction {
            label: None,
            directive: None,
            opcode: Some(Token::Op{code: OpCode::ADD}),
            operand1: Some(Token::Register{reg_num: 0}),
            operand2: Some(Token::Register{reg_num: 1}),
            operand3: Some(Token::Register{reg_num: 2})
          }
        )),
        result
      );
  }

  #[test]
  fn test_parse_hlt() {
    let result = instruction(CompleteStr("hlt    \n"));
    assert!(result.is_ok());
    let (_, ins) = result.unwrap();
    assert_eq!(
      AssemblerInstruction {
        label: None,
        directive: None,
        opcode: Some(Token::Op { code: OpCode::HLT }),
        operand1: None,
        operand2: None,
        operand3: None
      },
      ins
    );
  }
}
