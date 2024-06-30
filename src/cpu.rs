pub struct Cpu{
    pub register_a:u8, 
    pub register_x:u8,
    pub register_y:u8,
    pub stack_pointer:u8,
    pub pc:u16,
    pub status:u8
}

impl Cpu{
    pub fn new() -> Cpu{
        Cpu{
            register_a:0,
            register_x:0,
            register_y:0,
            stack_pointer:0xfd,
            pc:0,
            status:0
        }
    }

    pub fn interpret(&mut self , program:Vec<u8>){
        self.pc = 0;
        loop{
            let opcode = program[self.pc as usize];
            self.pc += 1;

            
            match opcode{

                //Break
                0x00 =>{
                    return;
                }
                // Implementing LDA
                0xA9 =>{
                    let params = program[self.pc as usize];
                    self.register_a = params;
                    if self.register_a == 0{
                        self.status = self.status | 0b0000_0010
                    }
                    else {
                        self.status = self.status & 0b1111_1101
                    }
                    if self.register_a & 0b1000_0000 !=0{
                        self.status = self.status | 0b1000_0000
                    }
                    else{
                        self.status = self.status | 0b0111_1111
                    }
                }
                //Implementing TAX
                0xAA =>{
                    self.register_x = self.register_a;
                    if self.register_x == 0{
                        self.status = self.status | 0b0000_0010
                    }
                    else {
                        self.status = self.status & 0b1111_1101
                    }
                    if self.register_x & 0b1000_0000 !=0{
                        self.status = self.status | 0b1000_0000
                    }
                    else{
                        self.status = self.status | 0b0111_1111
                    }
                }

                //Implementing INX
                0xe8=>{
                    self.register_x = self.register_x.wrapping_add(1); 
                    if self.register_x == 0{
                        self.status = self.status | 0b0000_0010
                    }
                    else {
                        self.status = self.status & 0b1111_1101
                    }
                    if self.register_x & 0b1000_0000 !=0{
                        self.status = self.status | 0b1000_0000
                    }
                    else{
                        self.status = self.status | 0b0111_1111
                    }
                }
                _=>{}
            }
        }
    }
}