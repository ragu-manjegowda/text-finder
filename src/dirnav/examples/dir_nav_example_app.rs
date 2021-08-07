use dir_nav::*;
use std::env::current_dir;
use std::io;
use std::path::{Path, PathBuf};

struct Appl {
    // Keep track of current directory
    current_directory: PathBuf,
    // Display directory name first before displaying file name
    display_directory: bool,
}

impl DirEvent for Appl {
    fn new() -> Self {
        Self {
            current_directory: PathBuf::new(),
            display_directory: true,
        }
    }

    fn do_dir(&mut self, directory: &Path) {
        self.current_directory = PathBuf::from(directory);
        self.display_directory = true;
    }

    fn do_file(&mut self, file_name: &Path) {
        if self.display_directory {
            println!("{}", self.current_directory.to_string_lossy());
            self.display_directory = false;
        }

        println!("{}", file_name.to_string_lossy());
    }
}

impl Default for Appl {
    fn default() -> Self {
        Self::new()
    }
}

fn main() -> io::Result<()> {
    let mut app = DirNav::<Appl>::new();

    app.add_patterns(Path::new("rs"));
    app.add_patterns(Path::new("rlib"));
    app.add_patterns(Path::new("d"));
    app.add_patterns(Path::new("toml"));

    println!("===== Non recursive search ===========");
    let _res = app.visit(Path::new(&current_dir()?))?;

    println!(
        "Processed {} directories and {} files",
        app.get_number_of_dirs_processed(),
        app.get_number_of_files_processed()
    );

    app.set_recursive_search();

    println!("===== Recursive search ===========");
    let _res = app.visit(Path::new(&current_dir()?))?;

    println!(
        "Processed {} directories and {} files",
        app.get_number_of_dirs_processed(),
        app.get_number_of_files_processed()
    );

    Ok(())
}
