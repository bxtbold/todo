use clap::{App, Arg};

fn main() {
    let matches = App::new("MyCLI")
        .version("1.0")
        .author("Batbold N.")
        .about("A simple todo-list CLI tool")
        .arg(
            Arg::with_name("greet")
                .short("g")
                .long("greet")
                .value_name("NAME")
                .help("Greet the specified name")
                .takes_value(true),
        )
        // Add more arguments/options as needed
        .get_matches();

    if let Some(name) = matches.value_of("greet") {
        println!("Hello, {}!", name);
    } else {
        println!("Hello, World!");
    }
}
