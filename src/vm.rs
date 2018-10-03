use instruction::OpCode;

pub struct VM {
  registers: [i32; 32],
  pc: usize,
  program: Vec<u8>,
  remainder: u32,
  equal_flag: bool
}

impl VM {
  pub fn new() -> VM {
    VM {
      registers: [0; 32],
      pc: 0,
      program: vec![],
      remainder: 0,
      equal_flag: false
    }
  }

  pub fn run(&mut self) {
    let mut is_done = false;
    while !is_done {
      is_done = self.execute_instruction();
    }
  }

  pub fn run_once(&mut self) {
    self.execute_instruction();
  }

  fn execute_instruction(&mut self) -> bool {
    if self.pc >= self.program.len() {
      // program counter above program length, we're done
      println!("Counter above program length, terminating");
      return false;
    }

    match self.decode_opcode() {
      OpCode::LOAD => {
        // Where the value must be loaded
        let register = self.next_8_bits();
        // the value to load
        let number = self.next_16_bits();
        self.registers[register as usize] = number as i32;
      },
      OpCode::ADD => {
        // 1st value
        let reg1 = self.next_8_bits();
        // 2nd value
        let reg2 = self.next_8_bits();
        // store result in the register at location from 3rd operan
        self.registers[self.next_8_bits() as usize] = reg1 as i32 + reg2 as i32;
      },
      OpCode::SUB => {
        let reg1 = self.next_8_bits();
        let reg2 = self.next_8_bits();
        self.registers[self.next_8_bits() as usize] = reg1 as i32 - reg2 as i32;
      },
      OpCode::MUL => {
        let reg1 = self.next_8_bits();
        let reg2 = self.next_8_bits();
        self.registers[self.next_8_bits() as usize] = reg1 as i32 * reg2 as i32;
      },
      OpCode::DIV => {
        let reg1 = self.next_8_bits();
        let reg2 = self.next_8_bits();
        // integer division (get the integer part)
        self.registers[self.next_8_bits() as usize] = reg1 as i32 / reg2 as i32;
        // remainder
        self.remainder = (reg1 % reg2) as u32;
      },
      OpCode::HLT => {
        println!("HLT encountered");
        return false;
      },
      OpCode::JMP => {
        let target = self.registers[self.next_8_bits() as usize];
        self.pc = target as usize;
      },
      OpCode::JMPF => {
        let target = self.registers[self.next_8_bits() as usize];
        self.pc += target as usize;
      },
      OpCode::JMPB => {
        let target = self.registers[self.next_8_bits() as usize];
        self.pc -= target as usize;
      },
      OpCode::EQ => {
        let reg1 = self.registers[self.next_8_bits() as usize];
        let reg2 = self.registers[self.next_8_bits() as usize];
        self.equal_flag = reg1 == reg2;
        self.next_8_bits();
      },
      OpCode::NEQ => {
        let reg1 = self.registers[self.next_8_bits() as usize];
        let reg2 = self.registers[self.next_8_bits() as usize];
        self.equal_flag = reg1 != reg2;
        self.next_8_bits();
      },
      OpCode::GT => {
        let reg1 = self.registers[self.next_8_bits() as usize];
        let reg2 = self.registers[self.next_8_bits() as usize];
        self.equal_flag = reg1 > reg2;
        self.next_8_bits();
      },
      OpCode::LT => {
        let reg1 = self.registers[self.next_8_bits() as usize];
        let reg2 = self.registers[self.next_8_bits() as usize];
        self.equal_flag = reg1 < reg2;
        self.next_8_bits();
      },
      OpCode::GTE => {
        let reg1 = self.registers[self.next_8_bits() as usize];
        let reg2 = self.registers[self.next_8_bits() as usize];
        self.equal_flag = reg1 >= reg2;
        self.next_8_bits();
      },
      OpCode::LTE => {
        let reg1 = self.registers[self.next_8_bits() as usize];
        let reg2 = self.registers[self.next_8_bits() as usize];
        self.equal_flag = reg1 <= reg2;
        self.next_8_bits();
      },
      OpCode::JEQ => {
        let target = self.registers[self.next_8_bits() as usize];
        if self.equal_flag {
          self.pc = target as usize;
        }
      },
      OpCode::JNEQ => {
        let target = self.registers[self.next_8_bits() as usize];
        if !self.equal_flag {
          self.pc = target as usize;
        }
      },
      _ => {
        println!("Unrecognized opcode. Terminating.");
        return false;
      }
    }
    return true;
  }

  fn decode_opcode(&mut self) -> OpCode {
    let opcode = OpCode::from(self.program[self.pc]);
    self.pc += 1;
    return opcode;
  }

  fn next_8_bits(&mut self) -> u8 {
    let result = self.program[self.pc];
    self.pc += 1;
    return result;
  }

  fn next_16_bits(&mut self) -> u16 {
    // get vector value on program as u16 and shift it 8 bits
    // then get the next vectory value
    // and combine the 2 into the result
    let result = ((self.program[self.pc] as u16) << 8) | (self.program[self.pc + 1]) as u16;
    // move the counter twice
    self.pc += 2;
    return result;
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn get_vm() -> VM {
    return VM::new();
  }

  #[test]
  fn create_vm() {
    let test_vm = get_vm();
    assert_eq!(0, test_vm.registers[0]);
    assert_eq!(0, test_vm.pc);
  }

  #[test]
  fn test_opcode_hlt() {
    let mut test_vm = get_vm();
    let test_code = vec![OpCode::HLT as u8, 0, 0, 0];
    test_vm.program = test_code;
    test_vm.run_once();
    assert_eq!(1, test_vm.pc);
  }

  #[test]
  fn test_opcode_ilg() {
    let mut test_vm = get_vm();
    let test_code = vec![254, 0, 0, 0];
    test_vm.program = test_code;
    test_vm.run_once();
    assert_eq!(1, test_vm.pc);
  }

  #[test]
  fn test_load_opcode() {
    let mut test_vm = get_vm();
    // last 2 operands are 1000 as 2u8 in hex format (3 * 16 * 16 and 14 * 16 + 8)
    let test_code = vec![OpCode::LOAD as u8, 0, 3, 232];
    test_vm.program = test_code;
    test_vm.run_once();
    assert_eq!(1000, test_vm.registers[0]);
  }

  #[test]
  fn test_add_opcode() {
    let mut test_vm = get_vm();
    let test_code = vec![OpCode::ADD as u8, 10, 15, 0];
    test_vm.program = test_code;
    test_vm.run_once();
    assert_eq!(25, test_vm.registers[0]);
  }

  #[test]
  fn test_sub_opcode() {
    let mut test_vm = get_vm();
    let test_code = vec![OpCode::SUB as u8, 10, 15, 0];
    test_vm.program = test_code;
    test_vm.run_once();
    assert_eq!(-5, test_vm.registers[0]);
  }

  #[test]
  fn test_mul_opcode() {
    let mut test_vm = get_vm();
    let test_code = vec![OpCode::MUL as u8, 10, 15, 0];
    test_vm.program = test_code;
    test_vm.run_once();
    assert_eq!(150, test_vm.registers[0]);
  }

  #[test]
  fn test_div_opcode() {
    let mut test_vm = get_vm();
    let test_code = vec![OpCode::DIV as u8, 17, 4, 0];
    test_vm.program = test_code;
    test_vm.run_once();
    assert_eq!(4, test_vm.registers[0]);
    assert_eq!(1, test_vm.remainder);
  }

  #[test]
  fn test_jmp_opcode() {
    let mut test_vm = get_vm();
    test_vm.registers[0] = 10;
    let test_code = vec![OpCode::JMP as u8, 0, 0, 0];
    test_vm.program = test_code;
    test_vm.run_once();
    assert_eq!(10, test_vm.pc);
  }

  #[test]
  fn test_jmpf_opcode() {
    let mut test_vm = get_vm();
    test_vm.registers[0] = 2;
    let test_code = vec![OpCode::JMPF as u8, 0, 0, 0, OpCode::HLT as u8, 0, 0, 0];
    test_vm.program = test_code;
    test_vm.run_once();
    assert_eq!(4, test_vm.pc);
  }

  #[test]
  fn test_jmpb_opcode() {
    let mut test_vm = get_vm();
    test_vm.registers[0] = 2;
    let test_code = vec![OpCode::JMPB as u8, 0, 0, 0];
    test_vm.program = test_code;
    test_vm.run_once();
    assert_eq!(0, test_vm.pc);
  }

  #[test]
  fn test_eq_opcode() {
    let mut test_vm = get_vm();
    test_vm.registers[0] = 2;
    test_vm.registers[1] = 2;
    test_vm.registers[2] = 2;
    test_vm.registers[3] = 3;
    let test_code = vec![OpCode::EQ as u8, 0, 1, 0, OpCode::EQ as u8, 2, 3, 0];
    test_vm.program = test_code;
    test_vm.run_once();
    assert!(test_vm.equal_flag);
    test_vm.run_once();
    assert!(!test_vm.equal_flag);
  }

  #[test]
  fn test_neq_opcode() {
    let mut test_vm = get_vm();
    test_vm.registers[0] = 0;
    test_vm.registers[1] = 3;
    test_vm.registers[2] = 0;
    test_vm.registers[3] = 0;
    let test_code = vec![OpCode::NEQ as u8, 0, 1, 0, OpCode::NEQ as u8, 2, 3, 0];
    test_vm.program = test_code;
    test_vm.run_once();
    assert!(test_vm.equal_flag);
    test_vm.run_once();
    assert!(!test_vm.equal_flag);
  }

  #[test]
  fn test_gt_opcode() {
    let code = OpCode::GT as u8;
    let mut test_vm = get_vm();
    test_vm.registers[0] = 3;
    test_vm.registers[1] = 0;
    test_vm.registers[2] = 0;
    test_vm.registers[3] = 2;
    let test_code = vec![code, 0, 1, 0, code, 2, 3, 0];
    test_vm.program = test_code;
    test_vm.run_once();
    assert!(test_vm.equal_flag);
    test_vm.run_once();
    assert!(!test_vm.equal_flag);
  }

  #[test]
  fn test_lt_opcode() {
    let code = OpCode::LT as u8;
    let mut test_vm = get_vm();
    test_vm.registers[0] = 0;
    test_vm.registers[1] = 3;
    test_vm.registers[2] = 2;
    test_vm.registers[3] = 0;
    let test_code = vec![code, 0, 1, 0, code, 2, 3, 0];
    test_vm.program = test_code;
    test_vm.run_once();
    assert!(test_vm.equal_flag);
    test_vm.run_once();
    assert!(!test_vm.equal_flag);
  }

  #[test]
  fn test_gte_opcode() {
    let code = OpCode::GTE as u8;
    let mut test_vm = get_vm();
    test_vm.registers[0] = 3;
    test_vm.registers[1] = 0;
    test_vm.registers[2] = 0;
    test_vm.registers[3] = 3;
    let test_code = vec![code, 0, 1, 0, code, 0, 1, 0, code, 2, 3, 0];
    test_vm.program = test_code;
    test_vm.run_once();
    assert!(test_vm.equal_flag);
    test_vm.registers[1] = 3;
    test_vm.run_once();
    assert!(test_vm.equal_flag);
    test_vm.run_once();
    assert!(!test_vm.equal_flag);
  }

  #[test]
  fn test_lte_opcode() {
    let code = OpCode::LTE as u8;
    let mut test_vm = get_vm();
    test_vm.registers[0] = 0;
    test_vm.registers[1] = 3;
    test_vm.registers[2] = 3;
    test_vm.registers[3] = 0;
    let test_code = vec![code, 0, 1, 0, code, 0, 1, 0, code, 2, 3, 0];
    test_vm.program = test_code;
    test_vm.run_once();
    assert!(test_vm.equal_flag);
    test_vm.registers[0] = 3;
    test_vm.run_once();
    assert!(test_vm.equal_flag);
    test_vm.run_once();
    assert!(!test_vm.equal_flag);
  }

  #[test]
  fn test_jeq_opcode() {
    let mut test_vm = get_vm();
    test_vm.registers[0] = 10;
    test_vm.equal_flag = true;
    let test_code = vec![OpCode::JEQ as u8, 0, 0, 0];
    test_vm.program = test_code;
    test_vm.run_once();
    assert_eq!(10, test_vm.pc);
  }

  #[test]
  fn test_jneq_opcode() {
    let mut test_vm = get_vm();
    test_vm.registers[0] = 10;
    test_vm.equal_flag = false;
    let test_code = vec![OpCode::JNEQ as u8, 0, 0, 0];
    test_vm.program = test_code;
    test_vm.run_once();
    assert_eq!(10, test_vm.pc);
  }
}
