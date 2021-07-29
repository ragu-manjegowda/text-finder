use dir_nav::DirEvent;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

pub trait SearchEvent {
    fn new() -> Self;
    fn set_dir(&mut self, dir: &Path);
    fn set_file(&mut self, result: (&Path, bool, &str));
}

pub struct TextSearch<T: SearchEvent> {
    dir: PathBuf,
    search_text: String,
    out: T,
    num_found: usize,
}

impl<T: SearchEvent> DirEvent for TextSearch<T> {
    fn new() -> Self {
        Self {
            dir: PathBuf::new(),
            search_text: String::new(),
            out: T::new(),
            num_found: 0,
        }
    }

    fn do_dir(&mut self, path: &Path) {
        self.dir = path.to_path_buf();
        self.out.set_dir(path);
    }

    fn do_file(&mut self, file_name: &Path) {
        let file_path = self.dir.join(file_name);

        let file = std::fs::File::open(file_path);

        if file.is_err() {
            self.out.set_file((file_name, false, "Cannot Open File"));
            return;
        }

        let mut file = file.unwrap();
        let mut contents = String::new();

        let result = file.read_to_string(&mut contents);

        if result.is_ok() {
            let found: bool = contents.contains(&self.search_text);

            if found {
                self.num_found += 1;
            }

            self.out.set_file((file_name, found, &self.search_text));
        }
    }
}

impl<T: SearchEvent> TextSearch<T> {
    pub fn set_search_text(&mut self, search_text: &str) {
        self.search_text = search_text.to_string();
    }

    pub fn get_app(&mut self) -> &mut T {
        &mut self.out
    }

    pub fn get_num_found(&self) -> usize {
        self.num_found
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockOutput {
        dir: PathBuf,
        file: PathBuf,
        search_text: String,
    }

    impl SearchEvent for MockOutput {
        fn new() -> Self {
            MockOutput {
                dir: PathBuf::new(),
                file: PathBuf::new(),
                search_text: String::new(),
            }
        }

        fn set_dir(&mut self, dir: &Path) {
            self.dir = dir.to_path_buf();
        }

        fn set_file(&mut self, result: (&Path, bool, &str)) {
            self.file = PathBuf::from(result.0);
            self.search_text = (result.2).to_string();
        }
    }

    #[test]
    fn test_sets() {
        let mut app = TextSearch::<MockOutput>::new();
        let search_text = "search_text";
        let search_path = Path::new("./src");

        app.set_search_text(search_text);
        app.do_dir(search_path);

        assert_eq!(app.search_text, search_text.to_string());
        assert_eq!(app.dir, search_path);
    }

    #[test]
    fn test_traits() {
        let mut app = TextSearch::<MockOutput>::new();
        let search_text = "SearchEvent";
        let search_path = Path::new("./src");
        let search_file = Path::new("text_search.rs");

        app.set_search_text(search_text);
        app.do_dir(search_path);
        app.do_file(search_file);

        assert_eq!(app.out.dir, search_path);
        assert_eq!(app.out.file, search_file);
        assert_eq!(app.out.search_text, search_text.to_string());
    }
}
