pub enum Instruction {
    ClearScreen,
    Jump,
    SetRegister,
    AddRegister,
    SetIdx,
    Draw,
    // TODO: remaining instructions
}
impl Instruction {
    pub fn from_op_code(op_code: u16) -> Result<Instruction, String> {
        todo!()
    }
}
