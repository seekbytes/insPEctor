use crate::pe::address::Address;
use crate::x86::instruction::Instruction;
use crate::x86::opcode::X86Opcode;
use crate::x86::operands::X86Operand;
use anyhow::{anyhow, bail, Result};
use pest::iterators::{Pair, Pairs};
use pest::Parser;

/// From a string that resembles an instruction, this trait returns the concrete types for
/// the instruction
pub struct InstructionParser {}

/// The grammar itself
#[allow(missing_docs)]
#[derive(pest_derive::Parser)]
#[grammar = "disassembler/grammar.pest"]
pub struct InstrParser;

impl InstructionParser {
    /// Parse an instruction
    pub fn parse(
        instruction: &str,
        address: Address,
        instruction_size: usize,
    ) -> Result<Instruction> {
        let pairs = InstrParser::parse(Rule::instruction, instruction)?;
        let mut instruction = None;

        for pair in pairs {
            match pair.as_rule() {
                Rule::instruction => {
                    let pairs = pair.into_inner();
                    instruction = Some(Self::build_inst(pairs, address, instruction_size));
                }
                _ => {
                    unreachable!()
                }
            }
        }

        match instruction {
            Some(instruction) => Ok(instruction?),
            None => Err(anyhow!("No instruction found")),
        }
    }

    /// Build instruction from a pair
    fn build_inst(
        pairs: Pairs<Rule>,
        address: Address,
        instruction_size: usize,
    ) -> Result<Instruction> {
        let mut opcode = X86Opcode::Invalid;
        let mut operands = vec![];
        for pair in pairs {
            match pair.as_rule() {
                Rule::opcode => {
                    let op = pair.as_str();
                    opcode = Self::retrieve_opcode(op);
                }
                Rule::operands => {
                    let pair_2 = pair.into_inner();
                    for p in pair_2 {
                        let ope = Self::build_ast(p);
                        operands.push(ope);
                    }
                }
                _ => {
                    bail!("got2: {}", pair)
                }
            }
        }

        let instr = Instruction::new(address, opcode, operands, instruction_size);
        Ok(instr)
    }

    /// Recursive function used to build an operand
    fn build_ast(pair: Pair<Rule>) -> X86Operand {
        match pair.as_rule() {
            Rule::register => {
                let reg_str = pair.as_str();
                // registers are hardcoded into grammars, so there's no need to insert a double
                // check. However, you can potentially improve this by checking which kind of x86
                // register is to transform some little registers (e.g. al) to concrete part of
                // long registers (e.g. al == slice(eax, 0, 15)
                X86Operand::Register(reg_str.to_string())
            }
            Rule::memory => {
                let inners_pair = pair.into_inner();

                let mut params = vec![];
                let mut length = 0;

                for p in inners_pair {
                    match p.as_rule() {
                        Rule::amount_mem => {
                            length = Self::retrieve_memory_size(p.as_str()).unwrap();
                        }
                        Rule::immediate => params.push(Self::build_ast(p)),
                        Rule::memory_expression => {
                            let inners = p.into_inner();
                            for p in inners {
                                match p.as_rule() {
                                    Rule::register => params.push(Self::build_ast(p)),
                                    Rule::displacement => {
                                        let mut multiplier = 1;
                                        for p in p.into_inner() {
                                            match p.as_rule() {
                                                Rule::register => {
                                                    params.push(Self::build_ast(p));
                                                }
                                                Rule::OP => {
                                                    if p.as_str() == "-" {
                                                        multiplier = 1;
                                                    }
                                                }
                                                Rule::immediate => {
                                                    let number = Self::build_ast(p);
                                                    let to_push = match number {
                                                        X86Operand::Immediate(immediate) => {
                                                            X86Operand::Immediate(
                                                                multiplier * immediate,
                                                            )
                                                        }
                                                        _ => unreachable!(),
                                                    };
                                                    params.push(to_push);
                                                }
                                                _ => {}
                                            }
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                        _ => {
                            panic!("get {}", p.as_str())
                        }
                    }
                }
                X86Operand::Memory { params, length }
            }
            Rule::immediate => {
                let number = pair.as_str().trim().parse::<i128>().unwrap();
                X86Operand::Immediate(number)
            }
            unknown => panic!("err: {:?}", unknown),
        }
    }

    /// Translate a fixed string into a concrete opcode
    fn retrieve_opcode(opcode: &str) -> X86Opcode {
        // TODO: this list is limited to the instructions we analyze from calc.exe
        match opcode {
            "xor" => X86Opcode::Xor,
            "xchg" => X86Opcode::Xchg,
            "test" => X86Opcode::Test,
            "sub" => X86Opcode::Sub,
            "shr" => X86Opcode::Shr,
            "shl" => X86Opcode::Shl,
            "setne" => X86Opcode::Setne,
            "sete" => X86Opcode::Sete,
            "sar" => X86Opcode::Sar,
            "ror" => X86Opcode::Ror,
            "rol" => X86Opcode::Rol,
            "ret" => X86Opcode::Ret,
            "rcl" => X86Opcode::Rcl,
            "out" => X86Opcode::Rut,
            "or" => X86Opcode::Or,
            "push" => X86Opcode::Push,
            "pop" => X86Opcode::Pop,
            "not" => X86Opcode::Not,
            "nop" => X86Opcode::Nop,
            "neg" => X86Opcode::Neg,
            "mov" => X86Opcode::Mov,
            "movabs" => X86Opcode::Movabs,
            "lock" => X86Opcode::Lock,
            "lea" => X86Opcode::Lea,
            "jrcxz" => X86Opcode::Jrcxz,
            "jno" => X86Opcode::Jno,
            "jne" => X86Opcode::Jne,
            "jmp" => X86Opcode::Jmp,
            "je" => X86Opcode::Je,
            "jl" => X86Opcode::Jl,
            "jbe" => X86Opcode::Jbe,
            "jb" => X86Opcode::Jb,
            "ja" => X86Opcode::Ja,
            "int3" => X86Opcode::Int3,
            "int" => X86Opcode::Int,
            "inc" => X86Opcode::Inc,
            "imul" => X86Opcode::Imul,
            "fstp" => X86Opcode::Fstp,
            "ficomp" => X86Opcode::Ficomp,
            "ficom" => X86Opcode::Ficom,
            "fcom" => X86Opcode::Fcom,
            "cmp" => X86Opcode::Cmp,
            "cmovne" => X86Opcode::Cmovne,
            "cdq" => X86Opcode::Cdq,
            "call" => X86Opcode::Call,
            "and" => X86Opcode::And,
            "add" => X86Opcode::Add,
            _ => X86Opcode::Invalid,
        }
    }

    /// Retrieve memory size for memory-based operands
    fn retrieve_memory_size(size: &str) -> Result<u64> {
        let mem_size = match size {
            "qword" => 64,
            "dword" => 32,
            "word" => 16,
            "byte" => 8,
            _ => {
                bail!("unrecognized memory size: {}", size);
            }
        };
        Ok(mem_size)
    }
}
