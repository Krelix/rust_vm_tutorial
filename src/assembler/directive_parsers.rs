use assembler::Token;
use assembler::instruction_parsers::AssemblerInstruction;
use assembler::operand_parsers::operand;
use nom::types::CompleteStr;
use nom::alpha1;

named!(directive_declaration<CompleteStr, Token>,
  do_parse!(
    tag!(".") >>
    name: alpha1 >>
    (
      Token::Directive{name: name.to_string()}
    )
  )
);

named!(directive_combined<CompleteStr, AssemblerInstruction>,
  do_parse!(
    tag!(".") >>
    name: directive_declaration >>
    o1: opt!(operand) >>
    o2: opt!(operand) >>
    o3: opt!(operand) >>
    (
      AssemblerInstruction {
        opcode: None,
        directive: Some(name),
        label: None,
        operand1: o1,
        operand2: o2,
        operand3: o3
      }
    )
  )
);

named!(pub directive<CompleteStr, AssemblerInstruction>,
  do_parse!(
    ins: alt!(directive_combined) >>
    ( ins )
  )
);
