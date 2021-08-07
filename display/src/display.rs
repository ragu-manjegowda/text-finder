use text_search::SearchEvent;

pub struct Display {
    /** Keep track of current directory */
    current_directory: std::path::PathBuf,
    /** Keep track of current file name */
    current_file: std::path::PathBuf,
    /** Text to be searched */
    search_text: String,
    /** Search text found in this file? */
    found: bool,
    /** Display directory? */
    display_directory: bool,
}

impl SearchEvent for Display {
    fn new() -> Self {
        Display {
            current_directory: std::path::PathBuf::new(),
            current_file: std::path::PathBuf::new(),
            search_text: String::new(),
            found: false,
            display_directory: false,
        }
    }

    fn set_dir(&mut self, dir: &std::path::Path) {
        self.current_directory = std::path::PathBuf::from(dir);
        self.display_directory = true;
    }

    fn set_file(&mut self, result: (&std::path::Path, bool, &str)) {
        self.current_file = result.0.to_path_buf();
        self.found = result.1;
        self.search_text = result.2.to_string();

        if self.found {
            if self.display_directory {
                println!("{}", self.current_directory.to_string_lossy());
                self.display_directory = false;
            }

            println!("{}", self.current_file.to_string_lossy());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_dir() {
        let mut app = Display::new();

        assert_eq!(app.current_directory, std::path::PathBuf::from(""));

        app.set_dir(std::path::Path::new("./"));

        assert_eq!(app.current_directory, std::path::PathBuf::from("./"));
        assert_eq!(app.display_directory, true);

        app.set_file((std::path::Path::new("display.rs"), true, "Display"));

        assert_eq!(app.current_directory, std::path::PathBuf::from("./"));
        assert_eq!(app.display_directory, false);

        app.set_dir(std::path::Path::new("../"));

        assert_eq!(app.current_directory, std::path::PathBuf::from("../"));
        assert_eq!(app.display_directory, true);
    }

    #[test]
    fn test_set_file() {
        let mut app = Display::new();
        app.set_dir(std::path::Path::new("./"));

        assert_eq!(app.current_file, std::path::PathBuf::from(""));
        assert_eq!(app.search_text, "");
        assert_eq!(app.found, false);

        app.set_file((std::path::Path::new("display.rs"), true, "Display"));

        assert_eq!(app.current_file, std::path::PathBuf::from("display.rs"));
        assert_eq!(app.search_text, "Display");
        assert_eq!(app.found, true);

        app.set_file((std::path::Path::new("display2.rs"), false, "Display"));

        assert_eq!(app.current_file, std::path::PathBuf::from("display2.rs"));
        assert_eq!(app.search_text, "Display");
        assert_eq!(app.found, false);
    }
}
