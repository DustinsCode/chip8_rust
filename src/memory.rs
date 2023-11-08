
const RAM: usize = 4096;

pub struct Memory {
    ram: [u8; RAM],
    stack: [u16; 16]
}

impl Memory {
    pub fn new() -> Self {

        let mut ram = [0u8; RAM];

        Memory {
            ram: ram,
            stack: [0; 16]
        }
    }
}