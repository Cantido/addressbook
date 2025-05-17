use std::fs;
use std::path::PathBuf;

use calcard::vcard::VCardProperty;
use calcard::Entry;
use clap::Parser;
use tabled::{builder::Builder, settings::Style};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Contact vcards to read
    vcards: Vec<PathBuf>,
}

fn main() {
    let cli = Cli::parse();

    let mut table_builder = Builder::with_capacity(100, 2);

    table_builder.push_record(["Name", "Phone"]);

    for vcard_path in cli.vcards.iter() {
        let vcard_contents =
            fs::read_to_string(vcard_path).expect("Should have been able to read vcard file");

        let mut parser = calcard::Parser::new(&vcard_contents);

        loop {
            match parser.entry() {
                Entry::VCard(vcard) => table_builder.push_record([
                    vcard
                        .property(&VCardProperty::Fn)
                        .expect("Expected vcard entry to have an FN property")
                        .values
                        .first()
                        .unwrap()
                        .as_text()
                        .unwrap(),
                    vcard
                        .property(&VCardProperty::Tel)
                        .map_or("", |p| p.values.first().unwrap().as_text().unwrap()),
                ]),
                Entry::Eof => {
                    break;
                }
                _ => {
                    todo!()
                }
            }
        }
    }

    let mut table = table_builder.build();
    println!("{}", table.with(Style::sharp()));
}
