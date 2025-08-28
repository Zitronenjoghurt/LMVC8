use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
    pub struct CPUStepFlags: u8 {
        const HALT = 0b0000_0001;
    }
}

impl CPUStepFlags {
    #[inline(always)]
    pub fn is_halt(&self) -> bool {
        self.contains(CPUStepFlags::HALT)
    }
}
