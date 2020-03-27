mod notes;

use clap::{load_yaml, App};

fn main() {
    let yml = load_yaml!("args.yml");
    let matches = App::from(yml).get_matches();
    
    if let Some(add_matches) = matches.subcommand_matches("add") {
        let title = add_matches.value_of("title").unwrap();
        match notes::new_note(&title) {
            Ok(_x) => println!("Done."),
            Err(e) => panic!("{}", e)
        }
    }
}
