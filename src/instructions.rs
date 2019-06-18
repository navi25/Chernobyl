use crate::num;

#[derive(Debug, PartialEq, FromPrimitive)]
pub enum Opcode {
    HALT = 0,
    LOAD = 1,
    ADD = 2,
    SUB = 3,
    MUL = 4,
    DIV = 5,
    JMP = 6,
    JMPF = 7,
    JMPB = 8,
    EQ = 9,
    JEQ = 10,
    JNEQ = 11,
    ILLEGAL = 255
}

impl From<u8> for Opcode {
    fn from(v : u8) -> Opcode {
        let opcode = num::FromPrimitive::from_u8(v);
        return opcode.unwrap_or(Opcode::ILLEGAL);
    }
}

#[derive(Debug, PartialEq)]
pub struct Instruction {
    opcode : Opcode
}

impl Instruction {

    pub fn new(opcode : Opcode) -> Instruction {
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
        let opcode = Opcode::HALT;
        assert_eq!(opcode, Opcode::HALT);
    }

    #[test]
    fn test_create_instruction(){
        let instruction = Instruction::new(Opcode::HALT);
        assert_eq!(instruction.opcode, Opcode::HALT);
    }

    #[test]
    fn test_opcode_match(){
        assert_eq!(Opcode::HALT,Opcode::from(0));
        assert_eq!(Opcode::ILLEGAL,Opcode::from(255));
    }
}


