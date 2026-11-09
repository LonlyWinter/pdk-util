//! Test

use std::{fs::File, io::BufReader};

use crate::PDKResult;

use super::meta::{LibLef, TechLef};

#[test]
fn lef_parse_lib() -> PDKResult<()> {
    let file = File::open("data/sky130_fd_sc_hd.lef")?;
    let mut reader = BufReader::new(file);
    let _lef = LibLef::parse(&mut reader)?;
    Ok(())
}

#[test]
fn lef_parse_tech() -> PDKResult<()> {
    let file = File::open("data/sky130_fd_sc_hd__nom.tlef")?;
    let mut reader = BufReader::new(file);
    let _lef = TechLef::parse(&mut reader)?;
    Ok(())
}
