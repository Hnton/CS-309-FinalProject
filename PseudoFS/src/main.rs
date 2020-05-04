use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize, Debug)]
pub struct Block {
    blockID: i128,
    nextNode: i128,
    payload: String,
}

impl Block {

    pub fn buildBlock(blockID: i128, nextNode: i128, payload: String) -> Block {
        Block {
            blockID: blockID,
            nextNode: nextNode,
            payload: payload,
        }
    }

    fn to_JSON(&mut self) -> Result<()> {

        let j = serde_json::to_string_pretty(&self)?;

        println!("{}", j);

        Ok(())
    }

    fn from_JSON(s: String) -> Block {


        let back: Block = serde_json::from_str(&s).unwrap();

        println!("{:?}", back);

        back
    }

    
}


fn main() {
    
    let test = Block {
        blockID: 100,
        nextNode: 101,
        payload: String::from("TEST"),
    };

    let mut _test2 = Block::buildBlock(1, 2, String::from("Nooo"));
    
    let JSO = serde_json::to_string_pretty(&_test2).unwrap();

    _test2.to_JSON();

    Block::from_JSON(JSO);
    
    // println!("Welcome to the PseudoFS Shell\n");

    // loop {

    //     println!("Choose one of the following actions");
    //     println!("1. Create a new disk image");
    //     println!("2. Format a disk image");
    //     println!("3. Mount a formatted disk image");
    //     println!("4. Unmount a disk image");
    //     println!("5. Diagnostics");
    //     println!("6. Delete");
    //     println!("7. Cat");
    //     println!("8. LS");
    //     println!("9. Copyin");
    //     println!("10. Copyout");
    //     println!("11. Help");
    //     println!("12. exit\n");
    //     println!("Enter your selection: ");

    //     let mut input = String::new();
    //     let mut fileName = String::new();
    //     let mut fileContents = String::new();
    //     // let FileSystem fs = FileSystem:new();


    //     std::io::stdin().read_line(&mut input).expect("Failed to read line");

    //     match input.trim().as_ref() {
    //         // CREATE
    //         "1" => { 
    //             println!("1. Create a new disk image");
    //         }
    //         // FORMAT
    //         "2" => { 
    //             println!("Enter the disk image name (assume it is in the disk\\ folder): ");

    //             std::io::stdin().read_line(&mut fileName).expect("Failed to read line");

    //         }
    //         // MOUNT
    //         "3" => { 
    //             println!("Enter the disk image name (assume it is in the disks\\ folder): ");
                
    //             std::io::stdin().read_line(&mut fileName).expect("Failed to read line");

    //         } 
    //         // UNMOUNT
    //         "4" => { 
    //             println!("4. Unmount a disk image");
    //         }
    //         // DEBUG
    //         "5" => { 
    //             println!("5. Diagnostics");
    //         }
    //         // DELETE
    //         "6" => { 
    //             println!("Enter the file name to delete: ");

    //             std::io::stdin().read_line(&mut fileName).expect("Failed to read line");

    //         }
    //         // CAT
    //         "7" => { 
    //             println!("Enter the file name to display: ");

    //             std::io::stdin().read_line(&mut fileName).expect("Failed to read line");

    //         }
    //         // LS
    //         "8" => { 
    //             println!("8. LS");
    //         }
    //         // COPY IN
    //         "9" => { 
    //             println!("Enter the path to the file on your computer to read: ");

    //             std::io::stdin().read_line(&mut fileName).expect("Failed to read line");

    //         }
    //         // COPY OUT
    //         "10" => { 
    //             println!("Enter the name of the file to save (from ls output): ");

    //             std::io::stdin().read_line(&mut fileName).expect("Failed to read line");

    //         }
    //         // HELP
    //         "11" => { 
    //             println!("\nHELP CHARLES\n");
    //         }
    //         // EXIT
    //         "12" => { 
    //             println!("\nThank you for using PseudoFS Shell\n"); 
    //             {break; }  
    //         }
    //            _ => { 
    //                println!("\nINVALID\n");
    //         }


    //     }

    // }


}

