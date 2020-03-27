mod notes;

fn main() {
    match notes::new_note("Example Title 1") {
        Ok(_x) => println!("Done."),
        Err(e) => panic!("{}", e)
    }
}
