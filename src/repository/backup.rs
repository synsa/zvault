use super::{Repository, Chunk, RepositoryError};

use ::util::*;

use std::fs::{self, File};
use std::path::Path;


#[derive(Default, Debug)]
pub struct Backup {
    pub root: Vec<Chunk>,
    pub total_data_size: u64,
    pub changed_data_size: u64,
    pub new_data_size: u64,
    pub encoded_data_size: u64,
    pub new_bundle_count: usize,
    pub chunk_count: usize,
    pub avg_chunk_size: f32,
    pub date: i64,
    pub duration: f32,
    pub file_count: usize,
    pub dir_count: usize
}
serde_impl!(Backup(u8) {
    root: Vec<Chunk> => 0,
    total_data_size: u64 => 1,
    changed_data_size: u64 => 2,
    new_data_size: u64 => 3,
    encoded_data_size: u64 => 4,
    new_bundle_count: usize => 5,
    chunk_count: usize => 6,
    avg_chunk_size: f32 => 7,
    date: i64 => 8,
    duration: f32 => 9,
    file_count: usize => 10,
    dir_count: usize => 11
});


impl Repository {
    pub fn list_backups(&self) -> Result<Vec<String>, RepositoryError> {
        let mut backups = Vec::new();
        let mut paths = Vec::new();
        let base_path = self.path.join("backups");
        paths.push(base_path.clone());
        while let Some(path) = paths.pop() {
            for entry in try!(fs::read_dir(path)) {
                let entry = try!(entry);
                let path = entry.path();
                if path.is_dir() {
                    paths.push(path);
                } else {
                    let relpath = path.strip_prefix(&base_path).unwrap();
                    backups.push(relpath.to_string_lossy().to_string());
                }
            }
        }
        Ok(backups)
    }

    pub fn get_backup(&self, name: &str) -> Result<Backup, RepositoryError> {
        let mut file = try!(File::open(self.path.join("backups").join(name)));
        Ok(try!(msgpack::decode_from_stream(&mut file)))
    }

    pub fn save_backup(&mut self, backup: &Backup, name: &str) -> Result<(), RepositoryError> {
        let mut file = try!(File::create(self.path.join("backups").join(name)));
        Ok(try!(msgpack::encode_to_stream(backup, &mut file)))
    }

    pub fn restore_backup<P: AsRef<Path>>(&mut self, backup: &Backup, path: P) -> Result<(), RepositoryError> {
        let inode = try!(self.get_inode(&backup.root));
        try!(self.save_inode_at(&inode, path));
        Ok(())
    }

    pub fn create_full_backup<P: AsRef<Path>>(&mut self, path: P) -> Result<Backup, RepositoryError> {
        // Maintain a stack of folders still todo
        // Maintain a map of path->inode entries
        // Work on topmost stack entry
        //   If it is a file, create inode for it and put it in the map
        //   If it is a folder, list contents and put entries not in the map on the stack, folders last
        //   If it is a folder with no missing entries, create a directory inode, add it to the map, and remove all children from the map
        // If stack is empty create a backup with the last inode as root
        unimplemented!()
    }
}