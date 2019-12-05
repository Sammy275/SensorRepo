pub mod logs {
    use std::io;
    use std::fs;
    use std::fs::File;
    use std::process;
    
    
    // sign up functionality
    fn signup() -> String {
        println!("Enter your username");
        let mut username = String::new();
        io::stdin().read_line(&mut username)
            .expect("Enter command");
        let username: String = username.trim().parse().unwrap();
        if username.len() == 0 {
            println!("Enter username");
            process::exit(1);
        }
        let filename = format!("{}.txt",username);
        let filename2 = format!("{}dev.txt",username);
        println!("Enter your password");
        let mut pass = String::new();
        io::stdin().read_line(&mut pass);
        let pass: String = pass.trim().parse().unwrap();
        if pass.len() == 0 {
            println!("Enter password");
            process::exit(1);
        }
        File::create(&filename);
        File::create(&filename2);
        fs::write(&filename, pass)
            .expect("unable to write to file");
        login();
        return filename2;
    }


    // log in functionality
    fn login() -> String {
        println!("Please enter your username");
        let mut username = String::new();
        io::stdin().read_line(&mut username)
            .expect("Enter command");
        let username: String = username.trim().parse().unwrap();
        let filename = format!("{}.txt",username);
        let filename2 = format!("{}dev.txt",username);
        let f = match File::open(&filename) {
            Ok(file) => file,
            Err(e) => {
                println!("The system couldnt find the user");
                process::exit(1);
            },
        };
        println!("Please enter your password");
        let mut pass = String::new();
        io::stdin().read_line(&mut pass)
            .expect("Enter password");
        let pass: String = pass.trim().parse().unwrap();
        let content = fs::read_to_string(&filename)
            .expect("Something went wrong");
        if content != pass {
            println!("The password does not match");
            process::exit(1);
        }
        else {
            println!("Access granted");
        }    
        filename2
    }

}