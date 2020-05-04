use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::time::{SystemTime, Instant};

#[derive(Serialize, Deserialize, Debug)]
pub enum FileType {
    Free,
	File,
	Directory
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Inode {
    pub number: i128,
    pub typee: FileType,
    pub startBlock: i128,
    pub size: i128,
    pub cTime: SystemTime,
}

impl Inode {

    pub fn buildInode(number: i128, typee: FileType, startBlock: i128, size: i128, cTime: SystemTime) -> Inode {
        Inode {
            number: number,
            typee: typee,
            startBlock: startBlock,
            size: size,
            cTime: cTime,
        }
    }

    pub fn to_JSON(&mut self) -> Result<()> {

        let j = serde_json::to_string_pretty(&self)?;

        println!("{}", j);

        Ok(())
    }

    pub fn from_JSON(s: String) -> Inode {


        let back: Inode = serde_json::from_str(&s).unwrap();

        println!("{:?}", back);

        back
    }    
}
fn main() {
    println!("Hello, world!");
}