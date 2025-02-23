use std::{collections::HashMap, env::current_dir, iter::Iterator, path::PathBuf};

use clap::{App, Arg, Error as ArgsError, ErrorKind as ArgsErrorKind};

#[derive(Debug)]
pub struct Arguments {
    pub directories: HashMap<String, PathBuf>,
}

impl Arguments {
    pub fn get() -> Result<Self, ArgsError> {
        let app = App::new("sqlite-vacuum")
            .arg(
                Arg::with_name("directory")
                    .value_name("DIRECTORY")
                    .multiple(true)
                    .takes_value(true)
                    .help("Sets the directories to walk")
                    .required(true),
            );

        let matches = app.get_matches_safe()?;

        let directories = if let Some(arg_values) = matches.values_of("directory") {
            arg_values
                .map(|value| (value.into(), PathBuf::from(value)))
                .collect()
        } else {
            let mut m = HashMap::with_capacity(1);
            m.insert(
                "".into(),
                current_dir().map_err(|error| ArgsError {
                    message: format!("Error accessing current working dir: {:?}", error),
                    kind: ArgsErrorKind::Io,
                    info: Some(vec!["directory".into()]),
                })?,
            );
            m
        };

        Ok(Self {
            directories,
        })
    }
}
