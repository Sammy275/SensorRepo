pub mod logs {
    use std::thread;
    use std::time::Duration;
    use std::fs;
    use std::fs::File;
    use std::process;
    
    use std::io::{self, prelude::*, BufReader};

    // sign up functionality
    pub fn signup() -> String{
        println!("Enter your username");
        let mut username = String::new();
        io::stdin().read_line(&mut username)
            .expect("Enter command");
        let username: String = username.trim().parse().unwrap();
        if username.len() == 0 {println!("Enter username"); thread::sleep(Duration::from_secs(1)); process::exit(1);}
        let filename = format!("{}.txt",username);
        println!("Enter your password");
        let mut pass = String::new();
        io::stdin().read_line(&mut pass);
        let pass: String = pass.trim().parse().unwrap();
        if pass.len() == 0 {println!("Enter password"); thread::sleep(Duration::from_secs(1)); process::exit(1);}
        File::create(&filename);
        fs::write(&filename, pass)
            .expect("unable to write to file");
        login();
        return filename;
    }


    // log in functionality
    pub fn login() -> String {
        let file;
        let mut filename;
        println!("Please enter your username");
        loop {
        let mut username = String::new();
        io::stdin().read_line(&mut username)
            .expect("Enter command");
        let username: String = username.trim().parse().unwrap();
        filename = format!("{}.txt",username);
        file = match File::open(&filename) {
            Ok(file) => file,
            Err(_) => {
                println!("The system couldnt find the user, Enter again");
                thread::sleep(Duration::from_secs(1));
                // process::exit(1);
                continue;
            },
        };
        break;
        }
        let _reader = BufReader::new(file);
        println!("Please enter your password");
        loop {
        let mut pass = String::new();
        io::stdin().read_line(&mut pass)
            .expect("Enter password");
        let pass: String = pass.trim().parse().unwrap();
        let content: String = fs::read_to_string(&filename)
            .expect("Something went wrong");
        let vec: Vec<&str> = content.trim().split('\n').collect();
        if vec[0] == pass {
            println!("Access Granted");
        }
        else {
            println!("The password does not match, Enter again");
            thread::sleep(Duration::from_secs(1));
            continue;
            // process::exit(1);
        }
        break;
        }    
        filename
    }

}