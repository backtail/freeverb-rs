use crate::memory::mut_mem_slice::MutMemSlice;

#[derive(Clone, Copy)]
pub struct DelayLine {
    pub buffer: MutMemSlice,
    index: usize,
}

impl DelayLine {
    pub fn new(buffer: MutMemSlice) -> Self {
        Self { buffer, index: 0 }
    }

    pub fn read(&self) -> f32 {
        unsafe { self.buffer.get_unchecked(self.index) }
    }

    pub fn write_and_advance(&mut self, value: f32) {
        unsafe {
            self.buffer.assign_unchecked(self.index, value);
        }

        if self.index == self.buffer.length - 1 {
            self.index = 0;
        } else {
            self.index += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tools::mut_mem_slice::from_slice;

    #[test]
    fn write() {
        let mut buffer = [0_f32; 24];

        let mut delay_line = DelayLine::new(from_slice(&mut buffer[..]));

        for (i, val) in buffer.iter().enumerate() {
            delay_line.write_and_advance(i as f32);
            assert_eq!(*val, i as f32);
        }
    }

    #[test]
    fn read() {
        let mut buffer = [0_f32; 24];
        for (i, val) in buffer.iter_mut().enumerate() {
            *val = i as f32;
        }

        let mut delay_line = DelayLine::new(from_slice(&mut buffer[..]));

        for val in buffer {
            assert_eq!(val, delay_line.read());
            delay_line.index += 1;
        }
    }
}
