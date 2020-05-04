
use std::time::{SystemTime, Instant};

mod block;
// mod disk;
mod inode;
mod directory;

fn main() {
    
    let mut _test1 = block::Block::buildBlock(1, 2, String::from("Nooo"));
    
    let JSO_block = serde_json::to_string_pretty(&_test1).unwrap();

    _test1.to_JSON();

    block::Block::from_JSON(JSO_block);

    let now = SystemTime::now();
    let mut _test2 = inode::Inode::buildInode(1, inode::FileType::Free , 2, 12, now);

    let JSO_Inode = serde_json::to_string_pretty(&_test2).unwrap();


    _test2.to_JSON();

    inode::Inode::from_JSON(JSO_Inode);


    let mut _test3 = directory::Directory::new();

    println!("{}", _test3);


}