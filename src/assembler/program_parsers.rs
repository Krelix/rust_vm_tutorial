use nom::types::CompleteStr;
use assembler::instruction_parser::*;

#[derive(Debug, PartialEq)]
pub struct Program{
  instructions: Vec<AssemblerInstruction>
}

impl Program{
  pub fn to_bytes(&self) -> Vec<u8> {
    let mut program = vec![];
    for instruction in &self.instructions {
      program.append(&mut instruction.to_bytes());
    }
    program
  }
}

named!{pub program<CompleteStr, Program>,
  do_parse!(
    instructions: many1!(instruction) >> (
      Program{instructions}
    )
  )
}

#[cfg(test)]
mod tests {
  use super::*;
  
  #[test]
  fn test_parse_program_ok() {
    let result = program(CompleteStr("load $1 #200 load $5 #1"));
    assert!(result.is_ok());
    let (_, prog) = result.unwrap();
    assert_eq!(2, prog.instructions.len());
  }

  #[test]
  fn test_program_to_bytes() {
      let result = program(CompleteStr("load $1 #500"));
      assert!(result.is_ok());
      let (_, prog) = result.unwrap();
      let bytes = prog.to_bytes();
      assert_eq!(4, bytes.len());
      assert_eq!(0, bytes[0]);
      assert_eq!(1, bytes[1]);
      // 256 for 1st byte
      assert_eq!(1, bytes[2]);
      assert_eq!(244, bytes[3]);
  }
}