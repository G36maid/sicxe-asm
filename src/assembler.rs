use sicxe::frame::*;
use sicxe::frameformer::block::rearrange_blocks;
use sicxe::frameformer::literal::dump_literals;
use sicxe::frameformer::section::split_into_sections;
use sicxe::frameformer::symbol::resolve_symbols;
use sicxe::frameformer::translate::translate_to_record;

use crate::optimize::{optimize, optimize_parallel};
use rayon::prelude::*;

pub fn assemble(source: &str) -> Result<String, String> {
    let frames = source
        .lines()
        .enumerate()
        .par_bridge()
        .map(|(i, line)| Frame::from_source(line, i as u32 + 1))
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();

    let sections = split_into_sections(frames)
        .into_par_iter()
        .map(rearrange_blocks)
        .map(dump_literals)
        .map(resolve_symbols)
        .collect::<Result<Vec<_>, _>>()?
        .into_par_iter()
        .map(translate_to_record)
        .collect::<Result<Vec<_>, _>>()?;

    let code = sections
        .into_iter()
        .map(optimize)
        .collect::<Vec<_>>()
        .join("\n");

    Ok(code)
}

pub fn assemble_parallel(source: &str) -> Result<String, String> {
    let frames = source
        .lines()
        .enumerate()
        .par_bridge()
        .map(|(i, line)| Frame::from_source(line, i as u32 + 1))
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();

    let sections = split_into_sections(frames)
        .into_par_iter()
        .map(rearrange_blocks)
        .map(dump_literals)
        .map(resolve_symbols)
        .collect::<Result<Vec<_>, _>>()?
        .into_par_iter()
        .map(translate_to_record)
        .collect::<Result<Vec<_>, _>>()?;

    let code = sections
        .into_iter()
        .map(optimize_parallel)
        .collect::<Vec<_>>()
        .join("\n");

    Ok(code)
}