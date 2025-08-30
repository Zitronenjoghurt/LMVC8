use crate::console::components::bus::MemoryMapped;
use crate::console::components::input_controller::flags::InputControllerFlags;
use crate::console::input::ConsoleInput;
use crate::console::types::address::Address;
use crate::console::types::byte::Byte;

mod flags;

#[derive(Debug, Default, Clone)]
pub struct InputController {
    pub touch_input_x: Byte,
    pub touch_input_y: Byte,
    pub flags: InputControllerFlags,
}

impl InputController {
    pub fn input(&mut self, input: ConsoleInput) {
        match input {
            ConsoleInput::Up => self.flags.insert(InputControllerFlags::KEY_UP),
            ConsoleInput::Down => self.flags.insert(InputControllerFlags::KEY_DOWN),
            ConsoleInput::Left => self.flags.insert(InputControllerFlags::KEY_LEFT),
            ConsoleInput::Right => self.flags.insert(InputControllerFlags::KEY_RIGHT),
            ConsoleInput::A => self.flags.insert(InputControllerFlags::KEY_A),
            ConsoleInput::B => self.flags.insert(InputControllerFlags::KEY_B),
            ConsoleInput::Start => self.flags.insert(InputControllerFlags::KEY_START),
            ConsoleInput::Touch((x, y)) => {
                self.flags.insert(InputControllerFlags::TOUCH);
                self.touch_input_x = x.into();
                self.touch_input_y = y.into();
            }
        }
    }
}

impl MemoryMapped for InputController {
    #[inline(always)]
    fn read(&mut self, addr: Address) -> Byte {
        match u16::from(addr) {
            0x0000 => self.touch_input_x,
            0x0001 => self.touch_input_y,
            0x0002 => self.flags.bits().into(),
            _ => Byte::new(0),
        }
    }

    #[inline(always)]
    fn write(&mut self, addr: Address, value: Byte) {
        match u16::from(addr) {
            0x0000 => self.touch_input_x = value,
            0x0001 => self.touch_input_y = value,
            0x0002 => self.flags.set_bits(value.into()),
            _ => {}
        }
    }
}
