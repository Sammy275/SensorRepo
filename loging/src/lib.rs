pub mod logs {
    use std::fs;
    use std::fs::File;
    use std::process;
    use std::fs::OpenOptions;
    use std::io::{self, prelude::*, BufReader};    
    
    // sign up functionality
    pub fn signup() -> String{
        println!("Enter your username");
        let mut username = String::new();
        io::stdin().read_line(&mut username)
            .expect("Enter command");
        let username: String = username.trim().parse().unwrap();
        if username.len() == 0 {println!("Enter username"); process::exit(1);}
        let filename = format!("{}.txt",username);
        println!("Enter your password");
        let mut pass = String::new();
        io::stdin().read_line(&mut pass);
        let pass: String = pass.trim().parse().unwrap();
        if pass.len() == 0 {println!("Enter password"); process::exit(1);}
        File::create(&filename);
        fs::write(&filename, pass)
            .expect("unable to write to file");
        login();
        return filename;
    }


    // log in functionality
    pub fn login() -> String {
        println!("Please enter your username");
        let mut username = String::new();
        io::stdin().read_line(&mut username)
            .expect("Enter command");
        let username: String = username.trim().parse().unwrap();
        let filename = format!("{}.txt",username);
        let file = match File::open(&filename) {
            Ok(file) => file,
            Err(e) => {
                println!("The system couldnt find the user");
                process::exit(1);
            },
        };
        let reader = BufReader::new(file);
        println!("Please enter your password");
        let mut pass = String::new();
        io::stdin().read_line(&mut pass)
            .expect("Enter password");
        let pass: String = pass.trim().parse().unwrap();
        let content: String = fs::read_to_string(&filename)
            .expect("Something went wrong");
        let mut vec: Vec<&str> = content.trim().split('\n').collect();
        if vec[0] == pass {
            println!("Access Granted");
        }
        else {
            println!("The password does not match");
            process::exit(1);
        }    
        filename
    }

}