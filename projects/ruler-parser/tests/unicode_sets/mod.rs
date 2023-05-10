use std::fs::File;
use std::io::Write;
use pex_trie::generate::xid::{XID_CONTINUE, XID_START};
use pex_trie::UnicodeSet;

#[test]
fn test() {
    let mut here = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target");
    let mut files = File::create(here.join("unicode_sets.rs")).unwrap();
    let xid_start = UnicodeSet::new("XID_START").with_ranges(XID_START);
    files.write(xid_start.export_rust_code().unwrap().as_bytes()).unwrap();
    let xid_continue = UnicodeSet::new("XID_CONTINUE").with_ranges(XID_CONTINUE).with_chars(['_']);
    files.write(xid_continue.export_rust_code().unwrap().as_bytes()).unwrap();
}