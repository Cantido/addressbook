use std::fs;
use std::io::{self, stdout};
use std::path::PathBuf;

use calcard::vcard::{VCard, VCardProperty};
use calcard::Entry;
use clap::Parser;
use crossterm::tty::IsTty;
use log::error;
use miette::{Diagnostic, IntoDiagnostic};
use tabled::{builder::Builder, settings::Style};
use thiserror::Error;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Contact vCards to read
    vcards: Vec<PathBuf>,
}

#[derive(Error, Debug, Diagnostic)]
pub enum AddressBookError {
    #[error("Could not read source file {:?}", path)]
    #[diagnostic(code(addressbook::io_error))]
    Io {
        path: PathBuf,

        #[source]
        cause: io::Error,
    },
    #[error("vCard property {:?} is required", property.as_str())]
    #[diagnostic(code(addressbook::missing_required_property))]
    MissingRequiredProperty { property: VCardProperty },
}

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let cli = Cli::parse();

    let contacts: Vec<VCard> = cli
        .vcards
        .iter()
        .map(|path| match fs::read_to_string(path) {
            Err(e) => Err(AddressBookError::Io {
                path: path.to_path_buf(),
                cause: e,
            }),
            Ok(contents) => Ok(contents),
        })
        .filter_map(|result| match result.into_diagnostic() {
            Err(e) => {
                error!("{:?}", e);
                None
            }
            Ok(contents) => Some(contents),
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

    if contacts.len() == 0 {
        return Ok(());
    }

    let mut table_builder = Builder::with_capacity(contacts.len(), 2);

    table_builder.push_record(["Name", "Phone"]);

    contacts
        .iter()
        .filter_map(|vcard| {
            let name_result = vcard
                .property(&VCardProperty::Fn)
                .and_then(|p| p.values.first())
                .and_then(|p| p.as_text())
                .ok_or(AddressBookError::MissingRequiredProperty {
                    property: VCardProperty::Fn,
                });

            let phone = vcard.property(&VCardProperty::Tel).map_or("", |p| {
                p.values.first().and_then(|p| p.as_text()).unwrap_or("")
            });

            match name_result.into_diagnostic() {
                Ok(name) => Some([name, phone]),
                Err(e) => {
                    error!("{:?}", e);
                    None
                }
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
