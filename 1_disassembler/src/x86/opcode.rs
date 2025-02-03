/// Represents an opcode for Intel x86_64
#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub enum X86Opcode {
    #[default]
    /// An invalid opcode because it is not in this list
    Invalid,

    /// x86 opcode that represents xor instruction
    Xor,
    /// x86 opcode that represents xchg instruction
    Xchg,
    /// x86 opcode that represents test instruction
    Test,
    /// x86 opcode that represents sub instruction
    Sub,
    /// x86 opcode that represents shr instruction
    Shr,
    /// x86 opcode that represents shl instruction
    Shl,
    /// x86 opcode that represents setne instruction
    Setne,
    /// x86 opcode that represents sete instruction
    Sete,
    /// x86 opcode that represents sar instruction
    Sar,
    /// x86 opcode that represents ror instruction
    Ror,
    /// x86 opcode that represents rol instruction
    Rol,
    /// x86 opcode that represents ret instruction
    Ret,
    /// x86 opcode that represents rcl instruction
    Rcl,
    /// x86 opcode that represents rut instruction
    Rut,
    /// x86 opcode that represents or instruction
    Or,
    /// x86 opcode that represents push instruction
    Push,
    /// x86 opcode that represents pop instruction
    Pop,
    /// x86 opcode that represents not instruction
    Not,
    /// x86 opcode that represents neg instruction
    Neg,
    /// x86 opcode that represents nop instruction
    Nop,
    /// x86 opcode that represents mov instruction
    Mov,
    /// x86 opcode that represents movabs instruction
    Movabs,
    /// x86 opcode that represents lock instruction
    Lock,
    /// x86 opcode that represents lea instruction
    Lea,
    /// x86 opcode that represents jrcxz instruction
    Jrcxz,
    /// x86 opcode that represents jno instruction
    Jno,
    /// x86 opcode that represents jne instruction
    Jne,
    /// x86 opcode that represents jmp instruction
    Jmp,
    /// x86 opcode that represents jl instruction
    Jl,
    /// x86 opcode that represents je instruction
    Je,
    /// x86 opcode that represents jbe instruction
    Jbe,
    /// x86 opcode that represents jb instruction
    Jb,
    /// x86 opcode that represents ja instruction
    Ja,
    /// x86 opcode that represents int3 instruction
    Int3,
    /// x86 opcode that represents int instruction
    Int,
    /// x86 opcode that represents inc instruction
    Inc,
    /// x86 opcode that represents imul instruction
    Imul,
    /// x86 opcode that represents fstp instruction
    Fstp,
    /// x86 opcode that represents ficomp instruction
    Ficomp,
    /// x86 opcode that represents ficom instruction
    Ficom,
    /// x86 opcode that represents fcom instruction
    Fcom,
    /// x86 opcode that represents cmp instruction
    Cmp,
    /// x86 opcode that represents cmovne instruction
    Cmovne,
    /// x86 opcode that represents cdq instruction
    Cdq,
    /// x86 opcode that represents call instruction
    Call,
    /// x86 opcode that represents and instruction
    And,
    /// x86 opcode that represents add instruction
    Add,
}
