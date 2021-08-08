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
