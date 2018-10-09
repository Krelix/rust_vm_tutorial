#[derive(Debug,PartialEq, Clone, Copy)]
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
      0 => OpCode::LOAD,
      1 => OpCode::ADD,
      2 => OpCode::SUB,
      3 => OpCode::MUL,
      4 => OpCode::DIV,
      5 => OpCode::HLT,
      6 => OpCode::JMP,
      7 => OpCode::JMPF,
      8 => OpCode::JMPB,
      9 => OpCode::EQ,
      10 => OpCode::NEQ,
      11 => OpCode::GT,
      12 => OpCode::LT,
      13 => OpCode::GTE,
      14 => OpCode::LTE,
      15 => OpCode::JEQ,
      16 => OpCode::JNEQ,
      _ => OpCode::IGL
    }
  }
}

pub struct Instruction {
  opcode: OpCode
}

impl Instruction {
  pub fn new(opcode: OpCode) -> Instruction {
    Instruction {
      opcode
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_create_hlt() {
    let instruction = Instruction::new(OpCode::HLT);
    assert_eq!(OpCode::HLT, instruction.opcode);
  }

  #[test]
  fn test_create_ilg() {
    let instruction = Instruction::new(OpCode::IGL);
    assert_eq!(OpCode::IGL, instruction.opcode);
  }
}
