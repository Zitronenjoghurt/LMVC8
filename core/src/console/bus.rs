use crate::console::ram::RAM;
use crate::console::types::address::Address;
use crate::console::types::byte::Byte;

pub struct Bus<'a> {
    pub ram: &'a mut RAM,
}

impl<'a> Bus<'a> {
    pub fn tick(&mut self) {}

    pub fn read(&mut self, address: Address) -> Byte {
        self.tick();
        todo!()
    }

    pub fn write(&mut self, address: Address, value: Byte) {
        self.tick();
        todo!()
    }
}
