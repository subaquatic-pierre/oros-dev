pub enum PortNumber {
    QemuDebugExit = 0xf4,
}

impl From<PortNumber> for u16 {
    fn from(value: PortNumber) -> Self {
        value as u16
    }
}
