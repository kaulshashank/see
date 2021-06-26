use clap::{App, Arg};

mod file_io;
mod util;

fn main() {
    let matches = App::new("see")
        .version("0.1")
        .author("Shashank Kaul <kaulshashank96@gmail.com>")
        .about("Handy combination of ls and cat")
        .arg(
            Arg::new("list")
                .short('l')
                .long("list")
                .about("List directory information"),
        )
        .arg(Arg::new("path").about("Path to see."))
        .get_matches();

    let mut path = "";
    let mut isList = false;

    if let Some(l) = matches.value_of("path") {
        path = l;
    } else {
        println!("Please provide a path.");
        return;
    }

    if matches.is_present("list") {
        isList = true;
    }

    let (dirs, files) = file_io::read_dir(path);

    util::print::display_dirs(dirs, isList);
    util::print::display_file(files, isList);
}
