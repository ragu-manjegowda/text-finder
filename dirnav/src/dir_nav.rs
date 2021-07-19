use std::fs::{self, DirEntry};
use std::io;
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};

/// typedef of Vector of PathBuf to hold file extensions
type SearchPatterns = Vec<PathBuf>;

/// Trait Event to process subdirectories and files in a directory
pub trait DirEvent {
    fn new() -> Self;
    fn do_dir(&mut self, d: &Path);
    fn do_file(&mut self, f: &Path);
}

pub struct DirNav<App: DirEvent> {
    /// File Patterns to search
    patterns: SearchPatterns,

    /// Instance of implementation of Trait DirEvent
    app: App,

    /// Number of files processed
    num_file: usize,

    /// Number of directories processed
    num_dirs: usize,

    /// Boolean flag for recursive search
    recurse: bool,
}

impl<App: DirEvent + Default> DirNav<App> {
    pub fn new() -> Self
    where
        App: DirEvent + Default,
    {
        Self {
            patterns: SearchPatterns::new(),
            app: App::new(),
            num_file: 0,
            num_dirs: 0,
            recurse: false,
        }
    }

    /// Add file extensions to pattern Vec
    pub fn add_patterns(&mut self, pattern: &Path) {
        self.patterns.push(pattern.to_path_buf());
    }

    /// Get file extensions to look for
    pub fn get_patterns(&mut self) -> &mut SearchPatterns {
        &mut self.patterns
    }

    /// Get instance to App of trait DirEvent
    pub fn get_app(&mut self) -> &mut App {
        &mut self.app
    }

    /// Returns processed file count
    pub fn get_number_of_files_processed(&self) -> usize {
        self.num_file
    }

    /// Returns processed dirs count
    pub fn get_number_of_dirs_processed(&self) -> usize {
        self.num_dirs
    }

    /// Function to set visits as recursive
    pub fn set_recursive_search(&mut self) {
        self.recurse = true;
    }

    /// Reset to default state
    pub fn reset(&mut self) {
        self.patterns.clear();
        self.app = App::default();
        self.num_file = 0;
        self.num_dirs = 0;
        self.recurse = false;
    }

    /// DFS on given path, call do_dir and do_file
    pub fn visit(&mut self, dir: &Path) -> io::Result<()>
    where
        App: DirEvent,
    {
        self.app.do_dir(dir);
        self.num_dirs += 1;

        let mut sub_dirs = Vec::<PathBuf>::new();

        // if it is a directory
        if dir.is_dir() {
            // read the contents of directory
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();

                if path.is_dir() {
                    sub_dirs.push(path);
                } else {
                    self.num_file += 1;
                    if self.has_pattern(&entry) | self.patterns.is_empty() {
                        self.app.do_file(&Path::new(&entry.file_name()));
                    }
                }
            }

            // recurse into directory
            if self.recurse {
                for dir in sub_dirs {
                    self.visit(&dir)?;
                }
            }
            return Ok(());
        }

        Err(Error::new(ErrorKind::Other, "Not a directory"))
    }

    /// Check if the pattern exist in list of patterns to check
    pub fn has_pattern(&self, directory: &DirEntry) -> bool {
        let path = directory.path();
        let extension = path.extension();
        match extension {
            Some(exist) => self.patterns.contains(&PathBuf::from(exist)),
            None => false,
        }
    }
}

impl<App: DirEvent + Default> Default for DirNav<App> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;

    struct Test {
        results: Vec<PathBuf>,
    }

    impl DirEvent for Test {
        fn new() -> Self {
            Self {
                results: Vec::<PathBuf>::new(),
            }
        }

        fn do_dir(&mut self, _d: &Path) {
            // Do nothing
        }

        fn do_file(&mut self, f: &Path) {
            self.results.push(PathBuf::from(f));
        }
    }

    impl Default for Test {
        fn default() -> Self {
            Self::new()
        }
    }

    fn setup() {
        let _ = std::fs::create_dir("./test_dir");
        let _ = std::fs::create_dir("./test_dir/test_sub_dir1");
        let _ = std::fs::create_dir("./test_dir/test_sub_dir2");
        let _ = std::fs::create_dir("./test_dir/test_sub_dir3");
        let _ = std::fs::File::create("./test_dir/test_file.rs");
        let _ = std::fs::File::create("./test_dir/test_sub_dir1/test_file1.rs");
        let _ = std::fs::File::create("./test_dir/test_sub_dir1/test_file2.bin");
        let _ = std::fs::File::create("./test_dir/test_sub_dir2/test_file3.txt");
        let _ = std::fs::File::create("./test_dir/test_sub_dir3/test_file4.out");
    }

    fn norecurse() {
        let mut directory_nav = DirNav::<Test>::new();
        directory_nav.add_patterns(&Path::new("rs"));
        directory_nav.add_patterns(&Path::new("bin"));
        directory_nav.add_patterns(&Path::new("out"));

        let mut path = PathBuf::new();
        path.push("./test_dir".to_string());

        let res = directory_nav.visit(&path);
        assert!(res.is_ok(), "Visit received error {:#?}", res.err());

        let res = &directory_nav.get_app().results;

        let path = |s: &str| -> PathBuf { PathBuf::from(s) };

        assert!(res.contains(&path("test_file.rs")));
        assert_eq!(res.contains(&path("test_file1.rs")), false);
        assert_eq!(res.contains(&path("test_file2.bin")), false);
        assert_eq!(res.contains(&path("test_file3.txt")), false);
        assert_eq!(res.contains(&path("test_file4.out")), false);
    }

    fn recurse() {
        let mut directory_nav = DirNav::<Test>::new();
        directory_nav.add_patterns(&Path::new("rs"));
        directory_nav.add_patterns(&Path::new("bin"));
        directory_nav.add_patterns(&Path::new("out"));

        directory_nav.set_recursive_search();

        let mut path = PathBuf::new();
        path.push("./test_dir".to_string());

        let res = directory_nav.visit(&path);
        assert!(res.is_ok(), "Visit received error {:#?}", res.err());

        let res = &directory_nav.get_app().results;

        let path = |s: &str| -> PathBuf { PathBuf::from(s) };

        assert!(res.contains(&path("test_file.rs")));
        assert!(res.contains(&path("test_file1.rs")));
        assert!(res.contains(&path("test_file2.bin")));
        assert!(res.contains(&path("test_file4.out")));

        // txt was not part of pattern
        assert_eq!(res.contains(&path("test_file3.txt")), false);
    }

    fn searchpatterns() {
        let mut directory_nav = DirNav::<Test>::new();
        directory_nav.add_patterns(&Path::new("rs"));
        directory_nav.add_patterns(&Path::new("bin"));
        directory_nav.add_patterns(&Path::new("out"));

        assert_eq!(directory_nav.get_patterns().len(), 3);

        directory_nav.reset();

        assert_eq!(directory_nav.get_patterns().len(), 0);
    }

    fn teardown() {
        let _ = std::fs::remove_dir_all("./test_dir");
    }

    fn run_test<T>(test: T) -> ()
    where
        T: FnOnce() -> () + panic::UnwindSafe,
    {
        setup();

        let result = panic::catch_unwind(|| test());

        teardown();

        assert!(result.is_ok())
    }

    #[test]
    fn test_norecurse() {
        run_test(|| {
            norecurse();
        })
    }

    #[test]
    fn test_recurse() {
        run_test(|| {
            recurse();
        })
    }

    #[test]
    fn test_searchpatterns() {
        run_test(|| {
            searchpatterns();
        })
    }
}
