pub struct Cpu{
    register_a:u8, 
    register_x:u8,
    register_y:u8,
    stack_pointer:u8,
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
}