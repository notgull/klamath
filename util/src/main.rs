// Apache 2.0 License

use clap::{App, Arg, SubCommand};
use std::{fs, io::Error as IoError, path::PathBuf, str::FromStr, sync::Arc};

mod bootstrap;
mod colormap;
mod dmxgus;
mod genmidi;
mod playpal;
mod textures;

#[derive(Debug, Clone)]
pub enum Error {
    StaticMsg(&'static str),
    Io(Arc<IoError>),
    Yaml(Arc<serde_yaml::Error>),
}

impl From<IoError> for Error {
    #[inline]
    fn from(i: IoError) -> Error {
        Error::Io(Arc::new(i))
    }
}

impl From<serde_yaml::Error> for Error {
    #[inline]
    fn from(sy: serde_yaml::Error) -> Error {
        Error::Yaml(Arc::new(sy))
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
        .subcommand(
            SubCommand::with_name("dmxgus")
                .about("Generates the DMXGUS lump for GUS sound cards with limited memory")
                .arg(
                    Arg::with_name("config")
                        .required(true)
                        .index(1)
                        .value_name("CONFIG"),
                ),
        )
        .subcommand(
            SubCommand::with_name("texture1")
                .about("Generates the TEXTURE1 and PNAMES lumps")
                .arg(
                    Arg::with_name("pnames")
                        .index(1)
                        .required(true)
                        .value_name("PNAMES"),
                )
                .arg(
                    Arg::with_name("texture1")
                        .index(2)
                        .required(true)
                        .value_name("TEXTURE1"),
                )
                .arg(
                    Arg::with_name("flats")
                        .short("f")
                        .long("flats")
                        .required(true)
                        .value_name("FLATS"),
                )
                .arg(
                    Arg::with_name("patches")
                        .short("p")
                        .long("patches")
                        .required(true)
                        .value_name("PATCHES"),
                )
                .arg(
                    Arg::with_name("wadin")
                        .short("i")
                        .long("inwad")
                        .required(true)
                        .value_name("INWAD"),
                )
                .arg(
                    Arg::with_name("wadout")
                        .short("o")
                        .long("outwad")
                        .required(true)
                        .value_name("OUTWAD"),
                )
                .arg(
                    Arg::with_name("matdir")
                        .short("m")
                        .long("matdir")
                        .required(true)
                        .value_name("MATDIR"),
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
    } else if let Some(matches) = matches.subcommand_matches("dmxgus") {
        let config = matches.value_of_os("config").unwrap();
        return dmxgus::generate_dmxgus(config.as_ref());
    } else if let Some(matches) = matches.subcommand_matches("texture1") {
        let pnames = matches.value_of_os("pnames").unwrap();
        let texture1 = matches.value_of_os("texture1").unwrap();
        let flats: PathBuf = matches.value_of_os("flats").unwrap().into();
        let patches: PathBuf = matches.value_of_os("patches").unwrap().into();
        let wadinfo_in = matches.value_of_os("wadin").unwrap();
        let wadinfo_out = matches.value_of_os("wadout").unwrap();
        let matdir = matches.value_of_os("matdir").unwrap();

        return textures::process_textures(
            pnames.as_ref(),
            texture1.as_ref(),
            patches,
            flats,
            wadinfo_in.as_ref(),
            wadinfo_out.as_ref(),
            matdir.as_ref(),
        );
    }

    Err(Error::StaticMsg("Did not receive any arguments."))
}
