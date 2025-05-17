use std::fs;
use std::io::{self, stdout};
use std::path::PathBuf;

use anyhow::{anyhow, Result};
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

    /// Contact properties to print
    #[arg(short, long, value_parser = property_name_parser, default_values = vec!["FN", "TEL"])]
    property: Vec<VCardProperty>,
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
}

fn property_name_parser(s: &str) -> Result<VCardProperty> {
    VCardProperty::try_from(s.as_bytes()).map_err(|_e| anyhow!("Unrecognized property name"))
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

    let mut table_builder = Builder::with_capacity(contacts.len(), cli.property.len());

    let headers = cli.property.iter().map(|prop| prop.as_str());

    if stdout().is_tty() {
        table_builder.push_record(headers);
    }

    contacts
        .iter()
        .map(|vcard| {
            cli.property
                .iter()
                .map(|prop| {
                    vcard
                        .property(&prop)
                        .and_then(|p| p.values.first())
                        .and_then(|p| p.as_text())
                        .unwrap_or("")
                })
                .collect()
        })
        .for_each(|row: Vec<&str>| {
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
