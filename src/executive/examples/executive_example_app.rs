use executive::*;

fn main() {
    println!("=============================");
    println!("===== Text Finder ===========");

    println!(
        "Find more information at https://github.com/ragu-manjegowda/text-finder \
        \nExample usage : \
        \ncargo run --example executive_example_app -- --path ./ --pattern toml --text name --recurse true"
    );

    println!("=============================");

    let mut cl_arguments: Vec<String> = std::env::args().collect();

    // Mimic command line arguments if nothing is passed
    if cl_arguments.len() < 2 {
        cl_arguments.append(&mut vec![
            "--path".to_string(),
            "./".to_string(),
            "--pattern".to_string(),
            "toml".to_string(),
            "--text".to_string(),
            "name".to_string(),
            "--recurse".to_string(),
            "true".to_string(),
        ]);
    }

    let mut ex = Executive::new();
    ex.parse_cla(&cl_arguments);

    ex.display_parsed_params();

    println!(
        "Searching for {:?} in {}",
        ex.get_valuesvec_for_key("text").unwrap(),
        ex.get_valuesvec_for_key("pattern").unwrap()[0]
    );

    ex.initialize_dir_nav();
    ex.start_text_finder();
    println!("=============================");
}
