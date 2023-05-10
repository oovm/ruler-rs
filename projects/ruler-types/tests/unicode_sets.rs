use std::fs::{ create_dir_all, File,};
use std::io::Write;
use std::path::{Path, };

use pex_trie::generate::xid::{XID_CONTINUE, XID_START};
use pex_trie::UnicodeSet;
use ruler_types::Symbol;

#[test]
fn test() -> std::io::Result<()> {
    let here = Path::new(env!("CARGO_MANIFEST_DIR")).join("target");
    create_dir_all(&here)?;
    println!("Export at: {}", here.canonicalize()?.display());
    let mut files = File::create(here.join("unicode_sets.rs"))?;
    let xid_start = UnicodeSet::new("XID_START").with_ranges(XID_START);
    files.write(xid_start.export_rust_code().unwrap().as_bytes())?;
    let xid_continue = UnicodeSet::new("XID_CONTINUE").with_ranges(XID_CONTINUE).with_chars(['_']);
    files.write(xid_continue.export_rust_code().unwrap().as_bytes())?;
    Ok(())
}

#[test]
fn test_symbol() {
    let symbol = Symbol::new("a ");
    println!("{}", symbol);
}