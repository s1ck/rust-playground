struct CPU {
    registers: [u8; 16],
    position_in_memory: usize,
    memory: [u8; 4096],
    stack: [u16; 16],
    stack_pointer: usize,
}

impl CPU {
    fn read_opcode(&mut self) -> u16 {
        let p = self.position_in_memory;
        let op_byte_1 = self.memory[p] as u16;
        let op_byte_2 = self.memory[p + 1] as u16;

        self.position_in_memory += 2;

        let res = (op_byte_1 << 8) | op_byte_2;

        res
    }

    fn run(&mut self) {
        loop {
            let op_code = self.read_opcode();

            let c = ((op_code & 0xF000) >> 12) as u8;
            let x = ((op_code & 0x0F00) >> 8) as u8;
            let y = ((op_code & 0x00F0) >> 4) as u8;
            let d = ((op_code & 0x000F) >> 0) as u8;

            let nnn = (op_code & 0x0FFF) as u16;

            match (c, x, y, d) {
                (0, 0, 0, 0) => return,
                (0x8, _, _, 0x4) => self.add_xy(x, y),
                (0x8, _, _, 0x5) => self.sub_xy(x, y),
                (0x8, _, _, 0x6) => self.mul_xy(x, y),
                (0x8, _, _, 0x1) => self.or_xy(x, y),
                (0x2, _, _, _) => self.call(nnn),
                (0x0, _, _, _) => self.ret(),
                _ => todo!("op_code {:04x}", op_code),
            }
        }
    }

    fn call(&mut self, address: u16) {
        let sp = self.stack_pointer;
        let stack = &mut self.stack;

        if sp > stack.len() {
            panic!("Stack overflow!")
        }

        stack[sp] = self.position_in_memory as u16;
        self.stack_pointer += 1;
        self.position_in_memory = address as usize;
    }

    fn ret(&mut self) {
        if self.stack_pointer == 0 {
            panic!("Stack underflow!")
        }

        self.stack_pointer -= 1;
        let call_address = self.stack[self.stack_pointer];
        self.position_in_memory = call_address as usize;
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        let arg_1 = self.registers[x as usize];
        let arg_2 = self.registers[y as usize];

        let (val, overflow) = arg_1.overflowing_add(arg_2);
        self.registers[x as usize] = val;

        self.handle_overflow(overflow);
    }

    fn sub_xy(&mut self, x: u8, y: u8) {
        let arg_1 = self.registers[x as usize];
        let arg_2 = self.registers[y as usize];

        let (val, overflow) = arg_1.overflowing_sub(arg_2);
        self.registers[x as usize] = val;

        self.handle_overflow(overflow);
    }

    fn mul_xy(&mut self, x: u8, y: u8) {
        let arg_1 = self.registers[x as usize];
        let arg_2 = self.registers[y as usize];

        let (val, overflow) = arg_1.overflowing_mul(arg_2);
        self.registers[x as usize] = val;

        self.handle_overflow(overflow);
    }

    fn or_xy(&mut self, x: u8, y: u8) {
        self.registers[x as usize] |= self.registers[y as usize];
    }

    // the last register is used as a carry flag to
    // indicate if an operation lead to anoverflow
    fn handle_overflow(&mut self, overflow: bool) {
        if overflow {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }
    }
}

#[test]
fn run_cpu() {
    let mut cpu = CPU {
        registers: [0; 16],
        position_in_memory: 0,
        memory: [0; 4096],
        stack: [0; 16],
        stack_pointer: 0,
    };

    cpu.registers[0] = 5;
    cpu.registers[1] = 10;

    let mem = &mut cpu.memory;

    // define a function
    let add_twice: [u8; 6] = [
        0x80, 0x14, // add register 0 and 1
        0x80, 0x14, // add register 0 and 1
        0x00, 0xEE, // return
    ];

    // store function in memory
    mem[0x100..0x106].copy_from_slice(&add_twice);

    mem[0x000] = 0x21;
    mem[0x001] = 0x00; // 0x2100 call function at 100
    mem[0x002] = 0x21;
    mem[0x003] = 0x00; // 0x2100 call function at 100
    mem[0x004] = 0x00;
    mem[0x005] = 0x00; // 0x0000 to halt

    cpu.run();

    assert_eq!(cpu.registers[0], 45);
}
