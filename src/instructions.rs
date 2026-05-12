pub enum Instruction {
    ClearScreen,
    Jump(u16),
    SetRegister { reg: u8, address: u16 },
    AddRegister { reg: u8, address: u16 },
    SetI(u16),
    Draw { x: u8, y: u8, address: u8 },
    // TODO: remaining instructions
}
impl Instruction {
    pub fn from_op_code(op_code: u16) -> Option<Instruction> {
        let nib1 = op_code & 0xF000 >> 12;
        let nib2 = op_code & 0x0F00 >> 8;
        let nib3 = op_code & 0x00F0 >> 4;
        let nib4 = op_code & 0x000F;

        match (nib1, nib2, nib3, nib4) {
            (0x0, 0x0, 0xE, 0x0) => Some(Instruction::ClearScreen),
            (0x1, _, _, _) => {
                let address = op_code & 0x0FFF;
                Some(Instruction::Jump(address))
            }
            (0x6, x, _, _) => {
                let address = op_code & 0x00FF;
                Some(Instruction::SetRegister {
                    reg: x as u8,
                    address,
                })
            }
            (0x7, x, _, _) => {
                let address = op_code & 0x00FF;
                Some(Instruction::AddRegister {
                    reg: x as u8,
                    address,
                })
            }
            (0xA, _, _, _) => {
                let address = op_code & 0x0FFF;
                Some(Instruction::SetI(address))
            }
            (0xD, x, y, n) => Some(Instruction::Draw {
                x: x as u8,
                y: y as u8,
                address: n as u8,
            }),
            _ => None,
        }
    }
}
