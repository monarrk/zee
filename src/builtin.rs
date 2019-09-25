use std::env;
use std::path::Path;

pub fn cd(args: &Vec<&str>) {
    if args.len() < 2 {
        return;
    }
    
    let path = Path::new(&args[1]);

    match env::set_current_dir(&path) {
        Ok(e) => e,
        Err(_) => {
            println!("Failed to change dir...");
            return;
        }
    };
}

pub fn exit() {
    std::process::exit(0);
}
