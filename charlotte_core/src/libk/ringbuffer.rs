//! Ring buffer implementation for the kernel
//!

pub struct RingBuffer {
    buff: &'static mut [u8],
    buff_len: u64,
    writer: u64,
    reader: u64,
}

impl RingBuffer {
    pub fn new_from_buff(buff: &'static mut [u8]) -> Self {
        RingBuffer {
            buff_len: buff.len() as u64,
            buff,
            writer: 0,
            reader: 0,
        }
    }

    pub fn data_available(&self) -> u64 {
        self.writer - self.reader
    }

    pub fn write(&mut self, data: &[u8]) {
        if data.len() as u64 > self.buff_len {
            panic!("Data is too large for the buffer")
        }

        data.iter()
            .map(|b| -> () {
                let idx = (self.writer + 1) % self.buff_len;
                self.writer = idx;
                self.buff[idx as usize] = *b;
            })
            .collect()
    }

    pub fn read(&mut self, dest: &mut [u8]) {
        let mut i = 0;
        while i < dest.len() {
            let idx = (self.reader + 1) % self.buff_len;
            self.reader = idx;
            dest[i] = self.buff[idx as usize];
            i += 1;
        }
    }
}
