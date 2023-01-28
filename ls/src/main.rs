use arg_handler::{ArgHandler, Args};
use std::cmp;
use std::ffi::OsString;
use std::fs::{self, DirEntry, FileType, Permissions};
use std::io::{self, Write};
use std::os::unix::prelude::OsStrExt;
use std::path::Path;
use std::str::FromStr;
use std::time::SystemTime;
use termsize;

mod arg_handler;

struct FileSystemEntry {
    name: OsString,
    r#type: FileType,
    size: u64,
    permissions: Permissions,
    last_modified: Option<SystemTime>,
    last_accessed: Option<SystemTime>,
    created_at: Option<SystemTime>,
}

struct List {
    longest_name_len: usize,
    entries: Vec<FileSystemEntry>,
}

impl List {
    pub fn new() -> List {
        return Self {
            entries: vec![],
            longest_name_len: 0,
        };
    }

    pub fn fetch_entries(&mut self, args: &Args) -> Result<(), io::Error> {
        let dir = &args.path;
        let mut entries = vec![];
        if dir.is_dir() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                let name = entry.file_name();
                let metadata = entry.metadata()?;
                self.longest_name_len = cmp::max(self.longest_name_len, name.len());
                let fs_entry = FileSystemEntry {
                    name,
                    r#type: entry.file_type()?,
                    size: metadata.len(),
                    permissions: metadata.permissions(),
                    last_modified: Some(metadata.modified()?),
                    last_accessed: Some(metadata.accessed()?),
                    created_at: Some(metadata.created()?),
                };
                entries.push(fs_entry);
            }
        }
        self.entries = entries;
        Ok(())
    }

    pub fn print(&self) {
        // TODO: Print the entries properly of the directory
        let term_size = termsize::get();
        match term_size {
            Some(size) => {
                let mut character_count = 0;
                for entry in &self.entries {
                    let out = format!(
                        "{:<width$}",
                        entry.name.clone().into_string().unwrap(),
                        width = self.longest_name_len + 2
                    );
                    if character_count + out.len() >= size.cols.into() {
                        io::stdout().write_all("\r\n".as_bytes()).unwrap();
                        character_count = 0;
                    }

                    character_count += out.len();
                    io::stdout().write_all(out.as_bytes()).unwrap();
                }
            }
            None => {
                for entry in &self.entries {
                    let output = format!("{:?}\t", entry.name);
                    io::stdout().write_all(output.as_bytes()).unwrap();
                }
            }
        }
    }
}

fn main() {
    // TODO: Handle arguments correctly
    let args = ArgHandler::new().get_args().unwrap();
    let mut list = List::new();
    list.fetch_entries(&args).unwrap();
    list.print();
}
