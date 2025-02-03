use crate::pe::address::Address;
use crate::x86::opcode::X86Opcode;
use crate::x86::operands::X86Operand;

use anyhow::{anyhow, Result};
/// Represents a concrete instruction for insPEctor.
#[derive(Clone, Default, Debug)]
pub struct Instruction {
    /// address for the instruction
    pub address: Address,
    /// opcode for the instruction
    pub opcode: X86Opcode,
    /// list of operands
    pub operands: Vec<X86Operand>,
    /// instruction size
    pub instruction_size: usize,
}

impl Instruction {
    /// Creates a new instance of instruction
    pub fn new(
        address: Address,
        opcode: X86Opcode,
        operands: Vec<X86Operand>,
        instruction_size: usize,
    ) -> Self {
        Self {
            address,
            opcode,
            operands,
            instruction_size,
        }
    }

    /// Returns if an instruction changes control flow
    pub fn change_cfg(&self) -> bool {
        let opcodes = [
            X86Opcode::Jmp,
            X86Opcode::Ja,
            X86Opcode::Jno,
            X86Opcode::Jb,
            X86Opcode::Jbe,
            X86Opcode::Jl,
            //X86Opcode::Call,
            X86Opcode::Jne,
            X86Opcode::Je,
        ];
        opcodes.contains(&self.opcode)
    }

    /// Returns if an instruction is an unconditional jump
    pub fn is_jump(&self) -> bool {
        self.opcode == X86Opcode::Jmp
    }

    /// Returns if an instruction is a conditional jump
    pub fn is_conditional_jump(&self) -> bool {
        let opcodes = [
            X86Opcode::Ja,
            X86Opcode::Jno,
            X86Opcode::Jb,
            X86Opcode::Jbe,
            X86Opcode::Jl,
            X86Opcode::Jne,
            X86Opcode::Je,
        ];
        opcodes.contains(&self.opcode)
    }

    /// Get immediate target for instructions that change control flow
    pub fn get_target(&self) -> Result<Option<Address>> {
        if self.operands.is_empty() {
            return Ok(None);
        }

        match self.change_cfg() {
            true => {
                let target_op = &self.operands[0];
                match target_op {
                    X86Operand::Immediate(value) => {
                        Ok(Some(self.address.wrapping_add(*value as u64)))
                    }
                    _ => Ok(None),
                }
            }
            false => Err(anyhow!(
                "This instruction is not a control flow instruction."
            )),
        }
    }

    /// Returns if an instruction is a return
    pub fn is_ret(&self) -> bool {
        self.opcode == X86Opcode::Ret
    }
}
