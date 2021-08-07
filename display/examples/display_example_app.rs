use display::Display;
use text_search::SearchEvent;

fn main() {
    let mut app = Display::new();
    app.set_dir(std::path::Path::new("./"));
    app.set_file((
        std::path::Path::new("display_example_app.rs"),
        true,
        "Display",
    ));
}
