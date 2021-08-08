use cli_parser::*;

fn set_args(app: &mut CliParser) {
    let cl_arguments: Vec<String> = std::env::args().collect();

    println!("cl_arguments = {:?}", cl_arguments);
    app.set_params(&cl_arguments);
    app.parse_params();

    if app.get_parsed_params().is_empty() {
        let s = vec![
            "cliparser".to_string(),
            "--path".to_string(),
            "./".to_string(),
            "--version".to_string(),
            "0.1".to_string(),
        ];

        app.set_params(&s);
        app.parse_params();
    }

    app.display_params();
    app.display_parsed_params();
}

fn set_defaults(app: &mut CliParser) {
    app.add_params_key_value("path", "../");
    app.add_params_key_value("version", "0.2");
    app.add_params_key_value("file", "Cargo.toml");

    app.display_parsed_params();
}

fn main() {
    let mut app = CliParser::new();

    println!("Parsing arguments, using dummy values if CLA is empty");
    set_args(&mut app);

    println!("Setting default arguments");
    set_defaults(&mut app);
}
