use crate::delay_line::DelayLine;
use crate::memory::mut_mem_slice::MutMemSlice;

#[derive(Clone, Copy)]
pub struct AllPass {
    pub delay_line: DelayLine,
}

impl AllPass {
    pub fn new(buffer: MutMemSlice) -> Self {
        Self {
            delay_line: DelayLine::new(buffer),
        }
    }

    pub fn tick(&mut self, input: f32) -> f32 {
        let delayed = self.delay_line.read();
        let output = -input + delayed;

        let feedback = 0.5;

        self.delay_line
            .write_and_advance(input + delayed * feedback);

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tools::mut_mem_slice::from_slice;

    #[test]
    fn basic_ticking() {
        let mut buffer = [0.0_f32; 2];
        let mut allpass = AllPass::new(from_slice(&mut buffer[..]));
        assert_eq!(allpass.tick(1.0), -1.0);
        assert_eq!(allpass.tick(0.0), 0.0);
        assert_eq!(allpass.tick(0.0), 1.0);
        assert_eq!(allpass.tick(0.0), 0.0);
        assert_eq!(allpass.tick(0.0), 0.5);
        assert_eq!(allpass.tick(0.0), 0.0);
        assert_eq!(allpass.tick(0.0), 0.25);
    }
}
