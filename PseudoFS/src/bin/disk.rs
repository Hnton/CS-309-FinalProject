// A disk will be how the file system interacts with the underlying file. A disk instance is created
// when the file is mounted and will open the file for read/write. The file will stay opened through
// the disk structure as long as the file system is mounted. The fields in the disk structure listed
use serde_json::ser::to_string;
use std::fs::File;
use std::io::Read;
use std::fs;
use std::path::Path;
use std::ffi::OsStr;
use std::io::BufReader;
use std::io::prelude::*;


mod block;

// here can be added to the diagnostics output to show the number of block reads/writes
// (otherwise they aren’t needed).

pub struct Disk 
{
    pub disk_content: Vec<String>,
    pub file: String,
    pub mounted: bool,
    pub reads: i128,
    pub writes: i128, 
}

impl Disk {
    pub fn new() -> Disk{
        Disk {
            disk_content: Vec::<String>::new(),
            file: "".to_string(),
            mounted: false,
            reads: 0,
            writes: 0

        }
    }

    // Open the disk parameter. If successful and the superblock is valid, the file
    // system is mounted and the superblock instance is available. The superblock
    // instance stored in memory after reading from block 0 should have a list of all
    // free inodes and blocks from the disk (see design)
    pub fn open(&mut self, f: &std::string::String) -> bool {
        let x = std::fs::read_to_string(&f).ok();
        self.file = (&f).to_string();
        match x {
            Some(a) => {
                // let path = Path::new(&f);
                for line in a.lines()
                {
                    self.disk_content.push(line.to_string());
                }
                println!("tezt: {:?}", self.disk_content);
                self.mounted = true;
                true
            },
            None => false
        }
    }

    // Close the disk. All data write operations must be completed. If successful, the
    // file system is unmounted.
    pub fn close() -> bool {

        return false;
    }

    // Write the block parameter to the given block id on the given disk. Return true if
    // successful and false if not
    pub fn write(blockID: i64, b: block::Block) -> bool {

        return false;

    }
}

fn main() {
    // println!("Hello, world!");
    // let mut file = File::open("D:/GIT/projects/CS-309-FinalProject/PseudoFS/src/test.txt").expect("Can't open");
    let temp = "D:/GIT/projects/CS-309-FinalProject/PseudoFS/src/test.txt";
    let mut disk =  Disk::new();
    // let mut contents = String::new();
    // file.read_to_string(&mut contents).expect("Oops can't read file");
    // println!("File Contents:\n\n{} ", contents);
    disk.open(&temp.to_string());

}