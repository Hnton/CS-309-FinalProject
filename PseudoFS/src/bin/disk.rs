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
    pub fn new(f: String) -> Disk{
        Disk {
            disk_content: Vec::<String>::new(),
            file: f,
            mounted: false,
            reads: 0,
            writes: 0

        }
    }

    // Open the disk parameter. If successful and the superblock is valid, the file
    // system is mounted and the superblock instance is available. The superblock
    // instance stored in memory after reading from block 0 should have a list of all
    // free inodes and blocks from the disk (see design)
    pub fn open(&mut self, mut f: String) -> bool {
        let x = std::fs::read_to_string("test.txt").ok();
        println!(" x: {:?}", x);
        match x {
            Some(a) => {
                let path = Path::new(&f);
                // for line in a.lines()
                // {
                //     self.diskContent.append(&line);
                // }
                println!("test: {:?}", a.lines());
                let temp: Vec<&str> = a.lines().collect();
                println!("tezt: {:?}", temp);

                // self.diskContent.append(&mut a.lines().collect()); //might have to convert a.lines into a vector of strings
                let file = File::open(&f).expect("Unable to open");
                let mut reader = BufReader::new(file);
                for line in reader.lines(){

                }
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
    println!("Hello, world!");
    let mut file = File::open("test.txt").expect("Can't open");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Oops can't read file");
    println!("File Contents:\n\n{} ", contents);
    // let mut test = Disk::new((&x).to_string());
    // test.open((&x).to_string());
    // println!("{}", x.to_string());

}