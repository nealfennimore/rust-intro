use crate::utils;

#[derive(Debug, Copy, Clone)]
pub struct What(pub u8, pub u8, pub u8);

impl What {
    pub fn whatwhat(self) -> u16 {
        self.0 as u16 + self.1 as u16 + self.2 as u16
    }

    pub fn debug(self) {
        utils::logger::debug(self.0);
        utils::logger::debug(self.1);
        utils::logger::debug(self.2);
    }
}
