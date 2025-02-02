use cpu::Cpu;
mod cpu;


fn main() {
    println!("Hello, world!");
}


#[cfg(test)]
mod tests {
    use super::*;

   #[test]
   fn test_5_ops_working_together() {
       let mut cpu = Cpu::new();
       cpu.interpret(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);
 
       assert_eq!(cpu.register_x, 0xc1)
   }

    #[test]
    fn test_inx_overflow() {
        let mut cpu = Cpu::new();
        cpu.register_x = 0xff;
        cpu.interpret(vec![0xe8, 0xe8, 0x00]);

        assert_eq!(cpu.register_x, 1)
    }
}
