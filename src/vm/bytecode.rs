#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum Bytecode {
    LoadConst,
    DiceRoll,
}
impl TryFrom<u8> for Bytecode {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Bytecode::LoadConst),
            1 => Ok(Bytecode::DiceRoll),
            _ => Err(())
        }
    }
}
// TODO: use a macro to auto-generate TryFrom<u8> for Bytecode due to large number of variants