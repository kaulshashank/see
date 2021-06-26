pub fn display_dirs(dirs: Vec<String>, list: bool) {
    if list {
        for dir in dirs {
            println!("* {}", dir);
        }
    } else {
        for dir in dirs {
            print!("{} ", dir);
        }
    }
}

pub fn display_file(files: Vec<String>, list: bool) {
    if list {
        for file in files {
            println!("- {}", file);
        }
    } else {
        for file in files {
            print!("{} ", file);
        }
    }
}
