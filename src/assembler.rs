use sicxe::frame::*;
use sicxe::frameformer::block::rearrange_blocks;
use sicxe::frameformer::literal::dump_literals;
use sicxe::frameformer::section::split_into_sections;
use sicxe::frameformer::symbol::resolve_symbols;
use sicxe::frameformer::translate::translate_to_record;

use crate::optimize::optimize;

pub fn assemble(source: &str) -> Result<String, String> {
    let frames = source
        .lines()
        .enumerate()
        .map(|(i, line)| Frame::from_source(line, i as u32 + 1))
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();

    let sections = split_into_sections(frames)
        .into_iter()
        .map(rearrange_blocks)
        .map(dump_literals)
        .map(resolve_symbols)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("Error in resolving symbols: {}", e))?
        .into_iter()
        .map(translate_to_record)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("Error in translating to record: {}", e))?;

    let code = sections
        .into_iter()
        .map(optimize)
        .collect::<Vec<_>>()
        .join("\n");

    Ok(code)
}
