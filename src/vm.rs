use crate::instructions::*;


#[derive(Debug)]
pub struct VM{
    registers: [i32; 32], // defining array of 32 size with i32 data type
    program : Vec<u8>, // actual program bytes
    pc: usize, // program counter
    remainder : u32
}


impl VM{
    pub fn new() -> VM {
        VM {
            registers: [0; 32], // initialised the array with 0
            program: vec![],
            pc: 0,
            remainder: 0
        }
    }

    pub fn run(&mut self){
        let mut is_done = false;
        while !is_done {
            is_done = self.execute_instruction();
        }
    }

    pub fn run_once(&mut self){
        self.execute_instruction();
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
            },
            Opcode::ADD => {
                let register = self.next_8_bits() as usize;
                let num1 = self.next_8_bits() as u8;
                let num2 = self.next_8_bits() as u8;
                let sum = num1 + num2;
                self.registers[register] = sum as i32;
            },
            Opcode::SUB => {
                let register = self.next_8_bits() as usize;
                let num1 = self.next_8_bits() as u8;
                let num2 = self.next_8_bits() as u8;
                let diff = num1 - num2;
                self.registers[register] = diff as i32;
            },
            Opcode::MUL => {
                let register = self.next_8_bits() as usize;
                let num1 = self.next_8_bits() as u8;
                let num2 = self.next_8_bits() as u8;
                let product = num1 * num2;
                self.registers[register] = product as i32;

            },
            Opcode::DIV => {
                let register = self.next_8_bits() as usize;
                let num1 = self.next_8_bits() as u8;
                let num2 = self.next_8_bits() as u8;
                let quotient = num1 / num2;
                let remainder = num1%num2;
                self.registers[register] = quotient as i32;
                self.remainder = remainder as u32;
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

        false

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

    #[test]
    fn test_add_opcode() {
        let mut test_vm = VM::new();
        let add_u8 = Opcode::ADD as u8;
        let test_bytes = vec![add_u8,1,2,1]; // add and store in register 1, num1 = 2, num2 =1
        test_vm.program = test_bytes;
        test_vm.run_once();
        assert_eq!(test_vm.registers[1],3);
    }

    #[test]
    fn test_sub_opcode() {
        let mut test_vm = VM::new();
        let sub_u8 = Opcode::SUB as u8;
        let test_bytes = vec![sub_u8,1,2,1]; // add and store in register 1, num1 = 2, num2 =1
        test_vm.program = test_bytes;
        test_vm.run_once();
        assert_eq!(test_vm.registers[1],1);
    }

    #[test]
    fn test_mul_opcode() {
        let mut test_vm = VM::new();
        let mul_u8 = Opcode::MUL as u8;
        let test_bytes = vec![mul_u8,1,2,1]; // add and store in register 1, num1 = 2, num2 =1
        test_vm.program = test_bytes;
        test_vm.run_once();
        assert_eq!(test_vm.registers[1],2);
    }

    #[test]
    fn test_div_opcode() {
        let mut test_vm = VM::new();
        let div_u8 = Opcode::DIV as u8;
        let test_bytes = vec![div_u8,1,4,2]; // add and store in register 1, num1 = 2, num2 =1
        test_vm.program = test_bytes;
        test_vm.run_once();
        assert_eq!(test_vm.registers[1],2);
    }

    #[test]
    fn test_div_opcode_with_remainder() {
        let mut test_vm = VM::new();
        let div_u8 = Opcode::DIV as u8;
        let test_bytes = vec![div_u8,1,5,2]; // add and store in register 1, num1 = 2, num2 =1
        test_vm.program = test_bytes;
        test_vm.run_once();
        assert_eq!(test_vm.registers[1],2);
        assert_eq!(test_vm.remainder,1);
    }


}