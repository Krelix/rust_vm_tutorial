#[derive(Debug,PartialEq)]
pub enum OpCode {
  LOAD, // 0
  ADD, // 1
  SUB, // 2
  MUL, // 3
  DIV, // 4
  HLT, // 5
  JMP, // 6
  JMPF, // 7
  JMPB, // 8
  EQ, // 9
  NEQ, // 10
  GT, // 11
  LT, // 12
  GTE, // 13
  LTE, // 14
  JEQ, // 15
  JNEQ, // 16
  IGL // not used for now
}

impl From<u8> for OpCode {
  fn from(code : u8) -> Self {
    match code {
      0 => return OpCode::LOAD,
      1 => return OpCode::ADD,
      2 => return OpCode::SUB,
      3 => return OpCode::MUL,
      4 => return OpCode::DIV,
      5 => return OpCode::HLT,
      6 => return OpCode::JMP,
      7 => return OpCode::JMPF,
      8 => return OpCode::JMPB,
      9 => return OpCode::EQ,
      10 => return OpCode::NEQ,
      11 => return OpCode::GT,
      12 => return OpCode::LT,
      13 => return OpCode::GTE,
      14 => return OpCode::LTE,
      15 => return OpCode::JEQ,
      16 => return OpCode::JNEQ,
      _ => return OpCode::IGL
    }
  }
}

pub struct Instruction {
  opcode: OpCode
}

impl Instruction {
  pub fn new(opcode: OpCode) -> Instruction {
    Instruction {
      opcode: opcode
    }
  }
}

#[cfg(tests)]
mod tests {
  use super::*;

  fn test_create_hlt() {
    let instruction = Instruction::new(OpCode::HLT);
    assert_eq!(OpCode::HLT, instruction.opcode);
  }

  fn test_create_ilg() {
    let instruction = Instruction::new(OpCode::IGL);
    assert_eq!(OpCode::IGL, instruction.opcode);
  }
}
