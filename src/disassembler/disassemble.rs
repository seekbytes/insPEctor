use crate::disassembler::instruction_parser::InstructionParser;
use crate::pe::address::Address;
use crate::pe::file_read::FileRead;
use crate::x86::instruction::Instruction;
use crate::x86::opcode::X86Opcode;
use anyhow::{Result};
use std::io::{Cursor, Read, Seek, SeekFrom};
use nyxstone::{Nyxstone, NyxstoneConfig};

/// Implements linear sweep and recursive traversal disassembler.
pub struct Disassembler {}

impl Disassembler {
    /// Extract the instructions
    pub fn extract(
        file_read: FileRead,
        disassembler_strategy: DisassemblerStrategy,
    ) -> Result<Vec<Instruction>> {
        let nyxstone = Nyxstone::new("x86_64", NyxstoneConfig::default())?;
        match disassembler_strategy {
            DisassemblerStrategy::LinearSweep => Self::extract_linear(file_read, &nyxstone),
            DisassemblerStrategy::RecursiveTraversal => Self::extract_recursive(file_read, &nyxstone),
        }
    }

    /// Implement linear sweep: one instruction at a time, and the first byte of the new disassembled
    /// instruction is based on the last one successfully disassembled.
    fn extract_linear(file_read: FileRead, nyxstone: &Nyxstone) -> Result<Vec<Instruction>> {

        let mut instructions = vec![];

        // implement the linear sweep
        let end_address = file_read.instr_address as usize + file_read.size;
        let mut cursor = Cursor::new(&file_read.bytes[..end_address]);

        // this is our current ground truth!!
        let mut current_address = file_read.instr_address;
        let mut buffer_instruction = [0; MAXIMUM_SIZE_X86_INSTR];

        while current_address < (end_address - MAXIMUM_SIZE_X86_INSTR) as u64 {
            let mut instruction_size = 1;

            if cursor.seek(SeekFrom::Start(current_address)).is_ok() {
                cursor.read_exact(&mut buffer_instruction)?;

                let i = nyxstone
                    .disassemble_to_instructions(buffer_instruction.as_slice(), current_address, 1);
                match i {
                    Ok(i) if i.len() == 1 => {
                        println!("Addr: 0x{:x}\tInstr: {}", i[0].address, i[0].assembly);
                        instruction_size = i[0].bytes.len();

                        let instruction_parsed = InstructionParser::parse(
                            i[0].assembly.as_str(),
                            i[0].address,
                            i[0].bytes.len(),
                        )?;
                        instructions.push(instruction_parsed);
                    }
                    _ => println!("We have no instructions for address: {:x}", current_address),
                }
            }
            current_address += instruction_size as u64;
        }

        Ok(instructions)
    }

    /// Implement recursive traversal: when an instruction that changes the control flow have an
    /// address as operand. Due to over approximation of the disassembler phase, we can have
    /// addresses that point to invalid instructions.
    fn extract_recursive(file_read: FileRead, nyxstone: &Nyxstone) -> Result<Vec<Instruction>> {
        let mut instructions = vec![];
        let mut cursor =
            Cursor::new(&file_read.bytes[..file_read.instr_address as usize + file_read.size]);

        // global list to understand if, given an address, was already disassembled
        let mut worklist = vec![];

        Self::recursive_disassemble(
            &mut cursor,
            file_read.entrypoint,
            &mut worklist,
            &mut instructions,
            nyxstone,
        )?;

        Ok(instructions)
    }

    /// Function for calling the recursive disassembling
    fn recursive_disassemble(
        cursor: &mut Cursor<&[u8]>,
        start_address: Address,
        global_list: &mut Vec<Address>,
        instructions: &mut Vec<Instruction>,
        nyxstone: &Nyxstone,
    ) -> Result<()> {

        // Add the instruction address to the one already seen
        global_list.push(start_address);

        let mut continue_to_disassemble = true;
        let mut current_address = start_address;
        let mut buffer_instruction = [0; MAXIMUM_SIZE_X86_INSTR];

        while continue_to_disassemble {
            if cursor.seek(SeekFrom::Start(current_address)).is_ok() {
                cursor.read_exact(&mut buffer_instruction)?;

                let instruction = nyxstone
                    .disassemble_to_instructions(buffer_instruction.as_slice(), current_address, 1);
                match instruction {
                    Ok(instruction) if instruction.len() == 1 => {
                        println!("{:x}, {:?}", current_address, instruction[0].assembly);
                        let instr_parsed = InstructionParser::parse(
                            instruction[0].assembly.as_str(),
                            instruction[0].address,
                            instruction[0].bytes.len(),
                        )?;

                        if instr_parsed.change_cfg() {
                            if let Some(target) = instr_parsed.get_target()? {
                                let current_target =
                                    target + instr_parsed.instruction_size as Address;
                                if !global_list.contains(&current_target) {
                                    Self::recursive_disassemble(
                                        cursor,
                                        current_target,
                                        global_list,
                                        instructions,
                                        nyxstone
                                    )?;
                                }
                            }

                            if instr_parsed.is_conditional_jump() {
                                Self::recursive_disassemble(
                                    cursor,
                                    current_address + instr_parsed.instruction_size as Address,
                                    global_list,
                                    instructions,
                                    nyxstone
                                )?;
                            }
                        }

                        if instr_parsed.is_ret() || instr_parsed.opcode == X86Opcode::Jmp {
                            continue_to_disassemble = false;
                        }
                        current_address += instr_parsed.instruction_size as Address;
                        instructions.push(instr_parsed);
                    }
                    _ => {
                        println!("We have no instructions for address: {:x}", current_address);
                        current_address += 1;
                        continue_to_disassemble = false;
                    }
                }
            } else {
                continue_to_disassemble = false;
            }
        }

        Ok(())
    }
}

/// Maximum size for an x86 instruction
const MAXIMUM_SIZE_X86_INSTR: usize = 15;

/// What strategy to use for disassembler
pub enum DisassemblerStrategy {
    /// Disassemble one instruction linearly: when an instruction ends, the next one starts from
    /// the one just disassembled.
    LinearSweep,
    /// Disassemble one instruction and evaluate its semantics: if it's an instruction that changes
    /// control flow
    RecursiveTraversal,
}
