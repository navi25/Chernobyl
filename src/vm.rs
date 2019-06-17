pub struct VM{
    registers: [i32; 32] // defining array of 32 size with i32 data type
}


impl VM{
    pub fn new() -> VM {
        VM {
            registers: [0; 32] // initialised the array with 0
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_vm(){
        let test_vm = VM::new();
        assert_eq!(test_vm.registers[0],0)
    }
}