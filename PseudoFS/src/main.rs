
fn main() {
    
  

    println!("Welcome to the PseudoFS Shell\n");

    loop {

        println!("Choose one of the following actions");
        println!("1. Create a new disk image");
        println!("2. Format a disk image");
        println!("3. Mount a formatted disk image");
        println!("4. Unmount a disk image");
        println!("5. Diagnostics");
        println!("6. Delete");
        println!("7. Cat");
        println!("8. LS");
        println!("9. Copyin");
        println!("10. Copyout");
        println!("11. Help");
        println!("12. exit\n");
        println!("Enter your selection: ");

        let mut input = String::new();
        let mut fileName = String::new();
        let mut fileContents = String::new();
        // let FileSystem fs = FileSystem:new();


        std::io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim().as_ref() {
            // CREATE
            "1" => { 
                println!("1. Create a new disk image");
            }
            // FORMAT
            "2" => { 
                println!("Enter the disk image name (assume it is in the disk\\ folder): ");

                std::io::stdin().read_line(&mut fileName).expect("Failed to read line");

            }
            // MOUNT
            "3" => { 
                println!("Enter the disk image name (assume it is in the disks\\ folder): ");
                
                std::io::stdin().read_line(&mut fileName).expect("Failed to read line");

            } 
            // UNMOUNT
            "4" => { 
                println!("4. Unmount a disk image");
            }
            // DEBUG
            "5" => { 
                println!("5. Diagnostics");
            }
            // DELETE
            "6" => { 
                println!("Enter the file name to delete: ");

                std::io::stdin().read_line(&mut fileName).expect("Failed to read line");

            }
            // CAT
            "7" => { 
                println!("Enter the file name to display: ");

                std::io::stdin().read_line(&mut fileName).expect("Failed to read line");

            }
            // LS
            "8" => { 
                println!("8. LS");
            }
            // COPY IN
            "9" => { 
                println!("Enter the path to the file on your computer to read: ");

                std::io::stdin().read_line(&mut fileName).expect("Failed to read line");

            }
            // COPY OUT
            "10" => { 
                println!("Enter the name of the file to save (from ls output): ");

                std::io::stdin().read_line(&mut fileName).expect("Failed to read line");

            }
            // HELP
            "11" => { 
                println!("\nHELP CHARLES\n");
            }
            // EXIT
            "12" => { 
                println!("\nThank you for using PseudoFS Shell\n"); 
                {break; }  
            }
               _ => { 
                   println!("\nINVALID\n");
            }


        }

    }


}

