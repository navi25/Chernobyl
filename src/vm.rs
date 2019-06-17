use crate::instructions::*;

#[derive(Debug)]
pub struct VM{
    registers: [i32; 32], // defining array of 32 size with i32 data type
    program : Vec<u8>, // actual program bytes
    pc: usize // program counter
}


impl VM{
    pub fn new() -> VM {
        VM {
            registers: [0; 32], // initialised the array with 0
            program: vec![],
            pc: 0
        }
    }

    pub fn run(&mut self){
        let mut is_done = false;
        while !is_done {
            is_done = self.execute_instruction();
        }
    }

    pub fn execute_instruction(&mut self) -> bool{

        if self.pc >= self.program.len(){
            return true;
        }
        match self.decode_opcode() {
            Opcode::LOAD => {
                let register = self.next_8_bits() as usize;
                let number = self.next_16_bits() as u16;
                self.registers[register] = number as i32;
                return false;
            },
            Opcode::HALT => {
                println!("HLT Encountered!");
                return true;
            },
            _ => {
                println!("Unrecognised opcode found! Terminating... ");
                return true;
            }
        }

    }

    fn decode_opcode(&mut self) -> Opcode {
        let opcode = Opcode::from(self.program[self.pc]);
        self.pc += 1;
        return opcode;
    }

    fn next_8_bits(&mut self) -> u8 {
        let result = self.program[self.pc];
        self.pc+=1;
        return result;
    }

    fn next_16_bits(&mut self) -> u16 {
        let result = ((self.program[self.pc] as u16) << 8) | self.program[self.pc+1] as u16;
        self.pc+=2;
        return result;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_vm() {
        let test_vm = VM::new();
        assert_eq!(test_vm.registers[0],0);
    }

    #[test]
    fn test_create_hlt() {
        let mut test_vm = VM::new();
        let test_bytes = vec![0,0,0,0];
        test_vm.program = test_bytes;
        test_vm.run();
        assert_eq!(test_vm.pc,1);
    }

    #[test]
    fn test_create_igl() {
        let mut test_vm = VM::new();
        let test_bytes = vec![200,0,0,0];
        test_vm.program = test_bytes;
        test_vm.run();
        assert_eq!(test_vm.pc,1);
    }

    #[test]
    fn test_load_opcode() {
        let mut test_vm = VM::new();
        let test_bytes = vec![1,0,1,244]; // 1 - load; 0 - in 0 register; 1 244 = 500
        test_vm.program = test_bytes;
        test_vm.run();
        assert_eq!(test_vm.registers[0],500);

    }
}