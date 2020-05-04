// A disk will be how the file system interacts with the underlying file. A disk instance is created
// when the file is mounted and will open the file for read/write. The file will stay opened through
// the disk structure as long as the file system is mounted. The fields in the disk structure listed
use std::fs::File;
use std::io::Read;


// here can be added to the diagnostics output to show the number of block reads/writes
// (otherwise they aren’t needed).

pub struct Disk {
    pub diskContent: Vec<String>,
    pub file: std::fs::File,
    pub mounted: bool,
    pub reads: i128,
    pub writes: i128, 
}

impl Disk {

    // Open the disk parameter. If successful and the superblock is valid, the file
    // system is mounted and the superblock instance is available. The superblock
    // instance stored in memory after reading from block 0 should have a list of all
    // free inodes and blocks from the disk (see design)
    pub fn open(f: String) -> bool {

        return false;
    }

    // Close the disk. All data write operations must be completed. If successful, the
    // file system is unmounted.
    pub fn close(d: Disk) -> bool {

        return false;
    }

    // Write the block parameter to the given block id on the given disk. Return true if
    // successful and false if not
    pub fn write(d: Disk, blockID: i64, b: Block) -> bool {

        return false;

    }


}


fn main() {
    println!("Hello, world!");
}

