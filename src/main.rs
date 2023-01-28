use std::ffi::OsString;
use std::fs::{self, DirEntry, FileType, Permissions};
use std::io::{self, Write};
use std::os::unix::prelude::OsStrExt;
use std::path::Path;
use std::time::SystemTime;

struct FileSystemEntry {
    name: OsString,
    r#type: FileType,
    size: u64,
    permissions: Permissions,
    last_modified: Option<SystemTime>,
    last_accessed: Option<SystemTime>,
    created_at: Option<SystemTime>,
}

fn main() {
    let arguments = vec![];
    // TODO: Handle arguments correctly
    let path = Path::new(".");
    let entries = fetch_entries(path, arguments);
    match entries {
        Ok(entries) => print(entries),
        Err(e) => println!("An error occurred while fetching entries"),
    }
}

fn fetch_entries(dir: &Path, args: Vec<String>) -> Result<Vec<FileSystemEntry>, io::Error> {
    let mut entries = vec![];
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            let metadata = entry.metadata()?;
            let fs_entry = FileSystemEntry {
                name: entry.file_name(),
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
    Ok(entries)
}

fn print(entries: Vec<FileSystemEntry>) {
    // TODO: Print the entries properly of the directory
    for mut entry in entries {
        entry.name.push("\t");
        io::stdout().write_all(entry.name.as_bytes()).unwrap();
    }
}
