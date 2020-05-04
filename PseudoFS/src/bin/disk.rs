// A disk will be how the file system interacts with the underlying file. A disk instance is created
// when the file is mounted and will open the file for read/write. The file will stay opened through
// the disk structure as long as the file system is mounted. The fields in the disk structure listed
use std::fs::File;
use std::io::Read;
use std::fs;
use std::path::Path;
use std::ffi::OsStr;

mod block;
mod disk;


// here can be added to the diagnostics output to show the number of block reads/writes
// (otherwise they aren’t needed).

pub struct Disk 
{
    pub diskContent: Vec<String>,
    pub file: File,
    pub mounted: bool,
    pub reads: i128,
    pub writes: i128, 
}

impl Disk {
    pub fn new(File f) -> Disk{
        Disk {
            diskContent: Vec::<String>::new(),
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
    pub fn open(f: String) -> bool {
        Path path = f;
        i16 lineCount = path.lines().count();
        diskContent = new String[lineCount];
        file = new File(f);
        let reader = BufReader::new(f);
        for line in reader.lines(){

        }
        return false;
    }

    // Close the disk. All data write operations must be completed. If successful, the
    // file system is unmounted.
    pub fn close(d: disk::Disk) -> bool {

        return false;
    }

    // Write the block parameter to the given block id on the given disk. Return true if
    // successful and false if not
    pub fn write(d: disk::Disk, blockID: i64, b: block::Block) -> bool {

        return false;

    }
}
