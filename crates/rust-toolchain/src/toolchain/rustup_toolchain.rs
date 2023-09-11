// TODO: move to separate crate which depends on this crate

use crate::Toolchain;

pub struct RustupToolchain {
    toolchain: Toolchain,
}

impl RustupToolchain {
    pub fn active() -> Option<Self> {
        todo!()
    }

    pub fn installed() -> Vec<Self> {
        todo!()
    }
}

impl From<RustupToolchain> for Toolchain {
    fn from(value: RustupToolchain) -> Self {
        value.toolchain
    }
}
