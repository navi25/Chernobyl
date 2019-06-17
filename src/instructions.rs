use std::fs::read_to_string;

#[derive(Debug, PartialEq)]
pub enum Opcode {
    HALT,
    LOAD,
    ILLEGAL
}

impl From<u8> for Opcode {
    fn from(v : u8) -> Opcode {
        match v {
            0 => return Opcode::HALT,
            1 => return Opcode::LOAD,
            _ => return Opcode::ILLEGAL
        }
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

}


