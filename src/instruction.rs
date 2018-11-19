use nom::types::CompleteStr;

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
  ALOC, // 17
  INC, // 18
  DEC, // 19
  IGL // unknown codes
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
      17 => OpCode::ALOC,
      18 => OpCode::INC,
      19 => OpCode::DEC,
      _ => OpCode::IGL
    }
  }
}

impl<'a> From<CompleteStr<'a>> for OpCode {
  fn from(v: CompleteStr<'a>) -> OpCode {
    match v {
      CompleteStr("load") => OpCode::LOAD,
      CompleteStr("add") => OpCode::ADD,
      CompleteStr("sub") => OpCode::SUB,
      CompleteStr("mul") => OpCode::MUL,
      CompleteStr("div") => OpCode::DIV,
      CompleteStr("hlt") => OpCode::HLT,
      CompleteStr("jmp") => OpCode::JMP,
      CompleteStr("jmpf") => OpCode::JMPF,
      CompleteStr("jmpb") => OpCode::JMPB,
      CompleteStr("eq") => OpCode::EQ,
      CompleteStr("neq") => OpCode::NEQ,
      CompleteStr("gt") => OpCode::GT,
      CompleteStr("lt") => OpCode::LT,
      CompleteStr("gte") => OpCode::GTE,
      CompleteStr("lte") => OpCode::LTE,
      CompleteStr("jeq") => OpCode::JEQ,
      CompleteStr("jneq") => OpCode::JNEQ,
      CompleteStr("aloc") => OpCode::ALOC,
      CompleteStr("inc") => OpCode::INC,
      CompleteStr("dec") => OpCode::DEC,
      _ => OpCode::IGL,
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

  #[test]
  fn fn_test_from() {
      let result = OpCode::from(CompleteStr("load"));
      assert_eq!(OpCode::LOAD, result);
  }
}
