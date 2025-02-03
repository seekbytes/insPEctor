use anyhow::{anyhow, Result};
use InsPEctor::disassembler::disassemble::{Disassembler, DisassemblerStrategy};
use InsPEctor::pe::binary_parser::BinaryParser;

fn main() -> Result<()> {
    env_logger::init();

    let args = std::env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        return Err(anyhow!("usage: ./InsPEctor [pe_executable]"));
    }

    let file_pe = BinaryParser::read(&args[1])?;
    // let strategy = DisassemblerStrategy::LinearSweep;
    let strategy = DisassemblerStrategy::RecursiveTraversal;
    let _disassembled_instructions = Disassembler::extract(file_pe, strategy)?;
    Ok(())
}
