// Apache 2.0 License

use clap::{App, Arg, SubCommand};
use std::{fs, io::Error as IoError, rc::Rc, str::FromStr};

mod bootstrap;
mod colormap;
mod genmidi;
mod playpal;

#[derive(Debug, Clone)]
pub enum Error {
    StaticMsg(&'static str),
    Io(Rc<IoError>),
}

impl From<IoError> for Error {
    #[inline]
    fn from(i: IoError) -> Error {
        Error::Io(Rc::new(i))
    }
}

pub type Result<T = ()> = std::result::Result<T, Error>;

fn main() -> Result {
    // set up clap architecture
    let matches = App::new("Klamath Util")
        .version("0.1")
        .author("notgull <jtnunley01@gmail.com>")
        .about("Provides a variety of utility functions for building klamath.wad")
        .subcommand(
            SubCommand::with_name("bootstrap")
                .about("Generates the bootstrap.wad file necessary to get DeuTeX to run"),
        )
        .subcommand(SubCommand::with_name("playpal").about("Generates the PLAYPAL lump"))
        .subcommand(
            SubCommand::with_name("rm")
                .about("Removes a specific file or directory, but doesn't return an error")
                .arg(Arg::with_name("files").required(true).min_values(1)),
        )
        .subcommand(
            SubCommand::with_name("colormap")
                .about("Generates a colormap from the specified palette")
                .arg(Arg::with_name("r").required(false).index(1).value_name("R"))
                .arg(Arg::with_name("g").required(false).index(2).value_name("G"))
                .arg(Arg::with_name("b").required(false).index(3).value_name("B")),
        )
        .subcommand(
            SubCommand::with_name("genmidi")
                .about("Generates the GENMIDI lump for MIDI emulation")
                .arg(
                    Arg::with_name("basedir")
                        .required(true)
                        .index(1)
                        .value_name("BASEDIR"),
                ),
        )
        .get_matches();

    if let Some(_) = matches.subcommand_matches("bootstrap") {
        bootstrap::write_bootstrap()?;
        return Ok(());
    } else if let Some(_) = matches.subcommand_matches("playpal") {
        playpal::generate_palette()?;
        return Ok(());
    } else if let Some(matches) = matches.subcommand_matches("rm") {
        for file in matches.values_of_os("files").unwrap() {
            if let Ok(meta) = fs::metadata(file) {
                if meta.is_dir() {
                    fs::remove_dir_all(file)?;
                } else {
                    fs::remove_file(file)?;
                }
            }
        }

        return Ok(());
    } else if let Some(matches) = matches.subcommand_matches("colormap") {
        let rgb = if let (Some(r), Some(g), Some(b)) = (
            matches.value_of("r"),
            matches.value_of("g"),
            matches.value_of("b"),
        ) {
            [
                u8::from_str(r).expect("R is not a number"),
                u8::from_str(g).expect("G is not a number"),
                u8::from_str(b).expect("B is not a number"),
            ]
        } else {
            [0, 0, 0]
        };

        colormap::generate_colormap(rgb)?;

        return Ok(());
    } else if let Some(matches) = matches.subcommand_matches("genmidi") {
        let basedir = matches.value_of_os("basedir").unwrap();
        genmidi::generate_genmidi(basedir)?;

        return Ok(());
    }

    Err(Error::StaticMsg("Did not receive any arguments."))
}
