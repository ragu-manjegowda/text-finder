use cli_parser::CliParser;
use dir_nav::DirNav;
use display::Display;
use text_search::TextSearch;

pub struct Executive {
    #[allow(dead_code)]
    cli_parser: CliParser,
    #[allow(dead_code)]
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
        // Function set_params implicitly validates the Command Line Arguments
        self.cli_parser.set_params(args);

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
        if self
            .cli_parser
            .get_valuesvec_for_key("path")
            .unwrap()
            .is_empty()
        {
            self.cli_parser.add_params_key_value("path", "./");
        }

        if self
            .cli_parser
            .get_valuesvec_for_key("pattern")
            .unwrap()
            .is_empty()
        {
            // pattern is empty by default, search on all files
        }

        if self
            .cli_parser
            .get_valuesvec_for_key("recurse")
            .unwrap()
            .is_empty()
        {
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

        if self
            .cli_parser
            .get_valuesvec_for_key("text")
            .unwrap()
            .is_empty()
        {
            self.cli_parser.add_params_key_value("text", "");
        }
    }

    pub fn initialize_dir_nav(&mut self) {
        let patterns = self.cli_parser.get_valuesvec_for_key("pattern").unwrap();
        for pattern in patterns {
            self.dir_nav.add_patterns(std::path::Path::new(pattern));
        }

        if self.cli_parser.get_valuesvec_for_key("recurse").unwrap()[0] == "true" {
            self.dir_nav.set_recursive_search();
        }
    }

    pub fn start_text_finder(&mut self) {
        let texts = self.cli_parser.get_valuesvec_for_key("text").unwrap();
        let paths = self.cli_parser.get_valuesvec_for_key("path").unwrap();

        for text in texts {
            self.dir_nav.get_app().set_search_text(text);
            for path in paths {
                self.dir_nav.visit(std::path::Path::new(path)).unwrap();
            }
        }
    }
}

impl Default for Executive {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
