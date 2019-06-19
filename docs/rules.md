# Rules 

This document explains the rules to read a given instructions
and how it is parsed by the VM. Various Opcodes currently 
supported are:-

   - [HALT = 0](#halt)
   - [LOAD = 1](#load)
   - [ADD = 2](#add)
   - [SUB = 3](#sub)
   - [MUL = 4](#mul)
   - [DIV = 5](#div)
   - [JMP = 6](#jmp)
   - [JMPF = 7](#jmpf)
   - [JMPB = 8](#jmpb)
   - [EQ = 9](#eq)
   - [JEQ = 10](#jeq)
   - [JNEQ = 11](#jneq)
   - [ILLEGAL = 255](#illegal)

### HALT 
Whenever a HALT opcode is encountered, VM terminates the program
execution.

### LOAD
Whenever a LOAD opcode is encountered, 
- VM reads next 8 bits,and store it as register index,
- VM reads next 16 bits and load it in the given register index

### ADD
Whenever a ADD opcode is encountered,
- VM reads next 8 bits, and store it as register index,
- VM reads next 8 bits as num1 and next 8 bits as num2
- VM add the sum and store the result at the register index

### SUB
Whenever a SUB opcode is encountered,
- VM reads next 8 bits, and store it as register index,
- VM reads next 8 bits as num1 and next 8 bits as num2
- VM take the diff as (num1 - num2) and store the result at the register index.
- Note negative number is not supported as of now.

### MUL
Whenever a MUL opcode is encountered,
- VM reads next 8 bits, and store it as register index,
- VM reads next 8 bits as num1 and next 8 bits as num2
- VM take the product and store the result at the register index.
- Note negative number is not supported as of now.

### DIV
Whenever a DIV opcode is encountered,
- VM reads next 8 bits, and store it as register index,
- VM reads next 8 bits as num1 and next 8 bits as num2
- VM take the diff and store the quotient at the register index and store remainder in VM.remainder variable
- Note negative number is not supported as of now.


### JMP
Whenever a JMP opcode (Jump) is encountered,
- VM reads next 8 bits, and store it as target,
- if target is greater than program len, we skip else we set the program counter (pc) value to target.

### JMPF
Whenever a JMPF opcode (Jump Forward) is encountered,
- VM reads next 8 bits, and store it as target,
- if program len is smaller than target + program counter, we skip 
- else we increment the program counter (pc) value by target.

### JMPB
Whenever a JMPB opcode (Jump Backward) is encountered,
- VM reads next 8 bits, and store it as target,
- if  program counter - target is lesser than 0, we skip 
- else we decrement the program counter (pc) value by target.

### EQ
Whenever a EQ opcode (Equal) is encountered,
- VM reads next 8 bits as num1 and next 8 bits as num2
- if num1 is equal to num2, VM's equal_flag is set to true else false

### JEQ
Whenever a JEQ opcode (Jump if Equal) is encountered,
- VM reads next 8 bits as target.
- if VM's equal_flag is set to true, program counter is set to target, else we skip

### JNEQ
Whenever a JNEQ opcode (Jump if Not Equal) is encountered,
- VM reads next 8 bits as target.
- if VM's equal_flag is set to false, program counter is set to target, else we skip

### ILLEGAL
Whenever a ILLEGAL opcode (Illegal) is encountered, VM stops the program execution.
