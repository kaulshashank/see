mod file_io;
mod util;

fn main() {
    println!("\n\nDirectories in this path are:");

    let (dirs, files) = file_io::read_dir(".");

    for dir in dirs {
        util::print::display_dir(dir);
    }

    for file in files {
        util::print::display_file(file);
    }
}
