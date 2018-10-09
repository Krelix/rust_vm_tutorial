pub mod opcode_parsers;
pub mod register_parsers;
pub mod operand_parsers;
pub mod instruction_parser;
pub mod program_parsers;

use instruction::OpCode;

#[derive(Debug,PartialEq)]
pub enum Token {
  Op{code: OpCode},
  Register{reg_num: u8},
  IntegerOperand{value: i32}
}

