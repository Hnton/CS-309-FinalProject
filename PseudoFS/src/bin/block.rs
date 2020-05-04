use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize, Debug)]
pub struct Block {
    pub blockID: i128,
    pub nextNode: i128,
    pub payload: String,
}

impl Block {

    pub fn buildBlock(blockID: i128, nextNode: i128, payload: String) -> Block {
        Block {
            blockID: blockID,
            nextNode: nextNode,
            payload: payload,
        }
    }

    pub fn to_JSON(&mut self) -> Result<()> {

        let j = serde_json::to_string_pretty(&self)?;

        println!("{}", j);

        Ok(())
    }

    pub fn from_JSON(s: String) -> Block {


        let back: Block = serde_json::from_str(&s).unwrap();

        println!("{:?}", back);

        back
    }

    
}

fn main() {
    println!("Hello, world!");
}