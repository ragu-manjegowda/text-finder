use cli_parser::CliParser;
use dir_nav::DirNav;
use display::Display;
use text_search::TextSearch;

pub struct Executive {
    cli_parser: CliParser,
    dir_nav: DirNav<TextSearch<Display>>,
}

impl Executive {
    pub fn new() -> Executive {
        Executive {
            cli_parser: CliParser::new(),
            dir_nav: DirNav::<TextSearch<Display>>::new(),
        }
    }

    pub fn parse_cla(&mut self, args: &[String]) {
        self.cli_parser.set_params(args);
        // Function parse_params implicitly validates the Command Line Arguments
        self.cli_parser.parse_params();

        // Check if --help is passed
        if args.iter().any(|i| i == "--help") {
            std::panic!(
                "
                Usage:
                --path `path/to/target/dir`, default `./`
                --pattern `search/file/patterns`, default `empty`
                --text `text to search`, default `\"\"` (lists all files)
                --recurse `search recursively in sub directories`, default `false`
                "
            );
        }

        // Now set the default values for empty values
        if self.cli_parser.get_valuesvec_for_key("path").is_none() {
            self.cli_parser.add_params_key_value("path", "./");
        }

        if self.cli_parser.get_valuesvec_for_key("pattern").is_none() {
            // pattern is empty by default, search on all files
        }

        if self.cli_parser.get_valuesvec_for_key("recurse").is_none() {
            self.cli_parser.add_params_key_value("recurse", "false");
        }

        if self
            .cli_parser
            .get_valuesvec_for_key("recurse")
            .unwrap()
            .len()
            > 1
        {
            std::panic!(
                "
                `recurse` cannot take more than one argument,
                valid value is either `true` or `false`
                "
            );
        }

        if self.cli_parser.get_valuesvec_for_key("recurse").unwrap()[0] != "true"
            && self.cli_parser.get_valuesvec_for_key("recurse").unwrap()[0] != "false"
        {
            std::panic!("valid value for `recurse` is either `true` or `false`");
        }

        if self.cli_parser.get_valuesvec_for_key("text").is_none() {
            self.cli_parser.add_params_key_value("text", "");
        }
    }

    pub fn initialize_dir_nav(&mut self) {
        let patterns = self.cli_parser.get_valuesvec_for_key("pattern");
        for pattern in patterns.iter().flat_map(|v| v.iter()) {
            self.dir_nav.add_patterns(std::path::Path::new(pattern));
        }

        if self.cli_parser.get_valuesvec_for_key("recurse").unwrap()[0] == "true" {
            self.dir_nav.set_recursive_search();
        }
    }

    pub fn start_text_finder(&mut self) {
        let texts = self.cli_parser.get_valuesvec_for_key("text");
        let paths = self.cli_parser.get_valuesvec_for_key("path").unwrap();

        for text in texts.iter().flat_map(|v| v.iter()) {
            self.dir_nav.get_app().set_search_text(text);
            for path in paths {
                self.dir_nav.visit(std::path::Path::new(path)).unwrap();
            }
        }
    }

    pub fn get_valuesvec_for_key(&self, key: &str) -> core::option::Option<&Vec<String>> {
        self.cli_parser.get_valuesvec_for_key(key)
    }

    pub fn display_parsed_params(&self) {
        self.cli_parser.display_parsed_params()
    }
}

impl Default for Executive {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construction_default() {
        let mut ex = Executive::new();
        ex.parse_cla(&Vec::<String>::new());
        assert_eq!(
            ex.cli_parser.get_valuesvec_for_key("path").unwrap().len(),
            1
        );
        assert!(ex.cli_parser.get_valuesvec_for_key("pattern").is_none());
        assert_eq!(
            ex.cli_parser
                .get_valuesvec_for_key("recurse")
                .unwrap()
                .len(),
            1
        );
        assert_eq!(
            ex.cli_parser.get_valuesvec_for_key("text").unwrap().len(),
            1
        );

        assert_eq!(
            ex.cli_parser.get_valuesvec_for_key("path").unwrap()[0],
            "./"
        );
        assert_eq!(
            ex.cli_parser.get_valuesvec_for_key("recurse").unwrap()[0],
            "false"
        );
        assert_eq!(ex.cli_parser.get_valuesvec_for_key("text").unwrap()[0], "");
    }

    #[test]
    fn test_construction_custom() {
        let mut ex = Executive::new();
        let cl_arguments: Vec<String> = vec![
            "--path".to_string(),
            "./".to_string(),
            "--pattern".to_string(),
            "toml".to_string(),
            "--text".to_string(),
            "name".to_string(),
            "--recurse".to_string(),
            "true".to_string(),
        ];

        ex.parse_cla(&cl_arguments);
        ex.cli_parser.display_parsed_params();
        assert_eq!(
            ex.cli_parser.get_valuesvec_for_key("path").unwrap().len(),
            1
        );
        assert_eq!(
            ex.cli_parser
                .get_valuesvec_for_key("pattern")
                .unwrap()
                .len(),
            1
        );
        assert_eq!(
            ex.cli_parser
                .get_valuesvec_for_key("recurse")
                .unwrap()
                .len(),
            1
        );
        assert_eq!(
            ex.cli_parser.get_valuesvec_for_key("text").unwrap().len(),
            1
        );

        assert_eq!(
            ex.cli_parser.get_valuesvec_for_key("path").unwrap()[0],
            "./"
        );
        assert_eq!(
            ex.cli_parser.get_valuesvec_for_key("recurse").unwrap()[0],
            "true"
        );
        assert_eq!(
            ex.cli_parser.get_valuesvec_for_key("text").unwrap()[0],
            "name"
        );
        assert_eq!(
            ex.cli_parser.get_valuesvec_for_key("pattern").unwrap()[0],
            "toml"
        );
    }

    #[test]
    fn test_execution() {
        let mut ex = Executive::new();
        ex.parse_cla(&Vec::<String>::new());
        ex.initialize_dir_nav();
        ex.start_text_finder();

        // Atleast 1 file is processed no matter where we run from
        assert!(ex.dir_nav.get_number_of_files_processed() > 0);
    }
}
