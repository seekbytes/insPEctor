use std::fmt::{Display, Formatter};

/// Represent an operand based on ISA x86
#[derive(Clone, Debug)]
pub enum X86Operand {
    /// A x86 register
    Register(String),
    /// An immediate value
    Immediate(i128),
    /// A memory offset defined with other params and a length
    Memory {
        /// the list of parameters used to identify memory (according to ISA)
        params: Vec<X86Operand>,
        /// the length of memory read or written
        length: u64,
    },
}

impl Display for X86Operand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            X86Operand::Register(reg) => {
                write!(f, "{}", reg)
            }
            X86Operand::Immediate(imm) => {
                write!(f, "0x{:x}", imm)
            }
            X86Operand::Memory { params, length } => {
                write!(f, "[{:?}]_{}", params, length)
            }
        }
    }
}
