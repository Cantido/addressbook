use std::fs;
use std::io::stdout;
use std::path::PathBuf;

use calcard::vcard::{VCard, VCardProperty};
use calcard::Entry;
use clap::Parser;
use crossterm::tty::IsTty;
use log::error;
use tabled::{builder::Builder, settings::Style};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Contact vCards to read
    vcards: Vec<PathBuf>,
}

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let cli = Cli::parse();

    let contacts: Vec<VCard> = cli
        .vcards
        .iter()
        .map(|path| (path, fs::read_to_string(path)))
        .filter_map(|(path, result)| {
            if result.is_err() {
                error!("Error reading file at {}", path.display());
                None
            } else {
                Some(result.unwrap())
            }
        })
        .flat_map(|file_contents| {
            let mut file_contacts = Vec::new();
            let mut parser = calcard::Parser::new(&file_contents);

            loop {
                match parser.entry() {
                    Entry::VCard(vcard) => file_contacts.push(vcard),
                    Entry::Eof => {
                        break;
                    }
                    _ => {
                        todo!()
                    }
                }
            }

            file_contacts
        })
        .collect();

    let mut table_builder = Builder::with_capacity(contacts.len(), 2);

    table_builder.push_record(["Name", "Phone"]);

    contacts
        .iter()
        .filter_map(|vcard| {
            let name: Option<&str> = vcard
                .property(&VCardProperty::Fn)
                .and_then(|p| p.values.first())
                .and_then(|p| p.as_text());

            let phone = vcard.property(&VCardProperty::Tel).map_or("", |p| {
                p.values.first().and_then(|p| p.as_text()).unwrap_or("")
            });

            if name.is_some() {
                Some([name.unwrap(), phone])
            } else {
                error!("Error reading contact, FN is a required property");
                None
            }
        })
        .for_each(|row| {
            table_builder.push_record(row);
        });

    let mut table = table_builder.build();

    if stdout().is_tty() {
        table.with(Style::sharp())
    } else {
        table.with(Style::blank())
    };

    println!("{}", table);

    Ok(())
}
