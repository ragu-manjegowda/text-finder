use dir_nav::DirEvent;
use std::path::{Path, PathBuf};
use text_search::*;

struct Appl {
    /** Keep track of current directory */
    current_directory: PathBuf,
    /** Keep track of current file name */
    current_file: PathBuf,
    /** Text to be searched */
    search_text: String,
    /** Search text found in this file? */
    found: bool,
    /** Display directory? */
    display_directory: bool,
}

impl SearchEvent for Appl {
    fn new() -> Self {
        Appl {
            current_directory: PathBuf::new(),
            current_file: PathBuf::new(),
            search_text: String::new(),
            found: false,
            display_directory: true,
        }
    }

    fn set_dir(&mut self, directory: &Path) {
        self.current_directory = PathBuf::from(directory);
        self.display_directory = true;
    }

    fn set_file(&mut self, result: (&Path, bool, &str)) {
        self.current_file = PathBuf::from(result.0);
        self.found = result.1;
        self.search_text = (result.2).to_string();

        if self.found {
            if self.display_directory {
                println!("{}", self.current_directory.to_string_lossy());
                self.display_directory = false;
            }

            println!("{}", self.current_file.to_string_lossy());
        }
    }
}

fn main() {
    let mut app = TextSearch::<Appl>::new();

    println!("===== Note ===========");

    println!(
        "Path used in example app is relative, \
           it needs to be executed inside the \
           folder, `textsearch`"
    );

    println!("===== Search positive ===========");

    let search_text = "TextSearch";
    let search_path = Path::new("./src");
    let search_file = Path::new("text_search.rs");
    app.set_search_text(search_text);
    app.do_dir(search_path);
    app.do_file(search_file);

    let search_text = "Appl";
    let search_path = Path::new("./examples");
    let search_file = Path::new("text_search_example_app.rs");
    app.set_search_text(search_text);
    app.do_dir(search_path);
    app.do_file(search_file);

    println!("===== Search negative ===========");

    let search_text = "DoesNotExist";
    let search_path = Path::new("./src");
    let search_file = Path::new("text_search.rs");
    app.set_search_text(search_text);
    app.do_dir(search_path);
    app.do_file(search_file);
}
