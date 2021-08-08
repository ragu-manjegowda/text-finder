use std::collections::HashMap;

/** Public struct to parse command line instructions */
pub struct CliParser {
    args_vec: Vec<String>,
    args_dict: HashMap<String, Vec<String>>,
    has_parsed: bool,
}

impl CliParser {
    pub fn new() -> Self {
        CliParser {
            args_vec: Vec::<String>::new(),
            args_dict: HashMap::<String, Vec<String>>::new(),
            has_parsed: false,
        }
    }

    pub fn set_params(&mut self, params: &[String]) {
        self.args_vec = params.to_vec();
        self.has_parsed = false;
    }

    pub fn display_params(&self) {
        println!("\n=============================");
        println!("Command line arguments = {:?}", self.args_vec);
        println!("=============================\n");
    }

    pub fn get_params(&mut self) -> &Vec<String> {
        &self.args_vec
    }

    pub fn parse_params(&mut self) {
        if self.has_parsed {
            println!("Have parsed the current arguments already!");
            return;
        }

        // Arguments should be of the form `binary name`, `--key`, `value`
        // Example:
        //      1. ./cliparser --path ./
        //      2. cargo run cli_parser --path ./
        if self.args_vec.len() < 3 {
            return;
        }

        // remove first argument which is self
        self.args_vec.remove(0);

        if self.args_vec[0][0..2] != *"--" && self.args_vec[1][0..2] != *"--" {
            std::panic!("Arguments should be in the format `--key` `value`");
        }

        // In case of, `cargo run cli_parser --path ./`, when it reaches here
        // `self.args_vec[0]` is `cli_parser`
        if self.args_vec[0][0..2] != *"--" {
            self.args_vec.remove(0);
        }

        let mut arg_str = "";

        for arg in &self.args_vec {
            if arg[0..2] == *"--" {
                // key cannot be empty
                if arg.len() < 2 {
                    std::panic!("Key cannot be empty `--`, should be of format `--key`");
                }

                arg_str = &arg[2..];

                self.args_dict
                    .entry(arg[2..].to_string().to_lowercase())
                    .or_insert_with(Vec::<String>::new);
            } else {
                if arg[0..1] == *"-" {
                    std::panic!("Value cannot begin with reserved char `-`");
                }
                CliParser::insert_if_not_exist(&mut self.args_dict, &arg_str.to_lowercase(), arg);
            }
        }

        self.has_parsed = true;
    }

    pub fn display_parsed_params(&self) {
        println!("\n=============================");
        println!("Parsed command line arguments : \n");
        for (key, value) in &self.args_dict {
            println!("{} = {:?}", key, value);
        }
        println!("=============================\n");
    }

    pub fn get_parsed_params(&mut self) -> &HashMap<String, Vec<String>> {
        &self.args_dict
    }

    // Allow to set/add to `key` a new `value`
    pub fn add_params_key_value(&mut self, key: &str, value: &str) {
        // If not exist, create a dummy entry to avoid remove returning `None`
        self.args_dict
            .entry(key.to_string().to_lowercase())
            .or_insert_with(Vec::<String>::new);

        CliParser::insert_if_not_exist(&mut self.args_dict, &key.to_lowercase(), value);
    }

    fn insert_if_not_exist(dict: &mut HashMap<String, Vec<String>>, key: &str, value: &str) {
        let mut v = dict.remove(&key.to_string()).unwrap();

        match v.binary_search(&value.to_string()) {
            Ok(_) => {
                println!("\n`--{} {}` not added as it already exists", &*key, &*value)
            }
            Err(pos) => v.insert(pos, value.to_string()),
        }

        dict.insert(key.to_string(), v);
    }

    pub fn get_valuesvec_for_key(&self, key: &str) -> core::option::Option<&Vec<String>> {
        if self.args_dict.contains_key(key) {
            return Some(&self.args_dict[key]);
        }

        None
    }
}

impl Default for CliParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parameters() {
        let mut app = CliParser::new();
        let cl_arguments: Vec<String> = std::env::args().collect();

        println!("\nSetting command line arguments as params");
        app.set_params(&cl_arguments);
        assert_eq!(*app.get_params(), cl_arguments);
        app.display_params();

        println!("Setting empty vector as params");
        app.set_params(&Vec::<String>::new());
        assert!(app.get_params().is_empty());
        app.display_params();
    }

    #[test]
    fn test_parsing_empty() {
        let mut app = CliParser::new();

        let cl_arguments = Vec::<String>::new();
        app.set_params(&cl_arguments);
        app.parse_params();
        assert!(app.get_parsed_params().is_empty());
    }

    #[test]
    fn test_parsing_non_empty() {
        let mut app = CliParser::new();

        let s = vec![
            "cliparser".to_string(),
            "--path".to_string(),
            "./".to_string(),
            "--version".to_string(),
            "0.1".to_string(),
        ];

        app.set_params(&s);
        app.display_params();
        app.parse_params();
        app.display_parsed_params();

        assert_eq!(app.get_parsed_params().len(), 2);

        app.parse_params();
        assert_eq!(app.get_parsed_params().len(), 2);
    }

    #[test]
    #[should_panic]
    fn test_parsing_invalid_key() {
        let mut app = CliParser::new();

        let s = vec![
            "cliparser".to_string(),
            "path".to_string(),
            "./".to_string(),
            "--version".to_string(),
            "0.1".to_string(),
        ];

        app.set_params(&s);
        app.display_params();

        app.parse_params();
    }

    #[test]
    #[should_panic]
    fn test_parsing_invalid_value() {
        let mut app = CliParser::new();

        let s = vec![
            "cliparser".to_string(),
            "--path".to_string(),
            "./".to_string(),
            "-version".to_string(),
            "0.1".to_string(),
        ];

        app.set_params(&s);
        app.display_params();
        app.parse_params();
    }

    #[test]
    fn test_parsing_duplicate() {
        let mut app = CliParser::new();

        let s = vec![
            "cliparser".to_string(),
            "--path".to_string(),
            "./".to_string(),
            "--version".to_string(),
            "0.1".to_string(),
            "--version".to_string(),
            "0.2".to_string(),
            "--path".to_string(),
            "./".to_string(),
            "--path".to_string(),
            "../".to_string(),
        ];

        app.set_params(&s);
        app.display_params();
        app.parse_params();
        app.display_parsed_params();

        assert_eq!(app.get_parsed_params().len(), 2);
        assert_eq!(app.get_valuesvec_for_key("path").unwrap().len(), 2);
        assert_eq!(app.get_valuesvec_for_key("version").unwrap().len(), 2);
    }

    #[test]
    fn test_parsing_set_key_value() {
        let mut app = CliParser::new();

        let s = vec![
            "cliparser".to_string(),
            "--path".to_string(),
            "./".to_string(),
            "--version".to_string(),
            "0.1".to_string(),
        ];

        app.set_params(&s);
        app.display_params();
        app.parse_params();
        app.display_parsed_params();
        assert_eq!(app.get_parsed_params().len(), 2);

        app.add_params_key_value("path", "../");
        app.display_parsed_params();
        assert_eq!(app.get_parsed_params().len(), 2);
        assert_eq!(app.get_valuesvec_for_key("path").unwrap().len(), 2);
        assert_eq!(app.get_valuesvec_for_key("version").unwrap().len(), 1);
    }
}
