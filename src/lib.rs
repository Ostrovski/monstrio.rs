//! Set of high level IO methods.

#![crate_name = "monstrio"]

use std::io::{BufRead, BufReader};
use std::fs::File;

#[macro_use]
extern crate log;

extern crate multi_reader;


pub struct Input<R> {
    source: R,
}

pub type MReader<I> = multi_reader::MultiReader<File, I>;
pub type BMReader<I> = BufReader<MReader<I>>;

impl<R: BufRead> AsMut<R> for Input<R> {
    fn as_mut(&mut self) -> &mut R {
        &mut self.source
    }
}

impl<I: Iterator<Item = File>> Input<BMReader<I>> {
    pub fn files(files: I) -> Input<BMReader<I>> {
        Input { source: BMReader::new(MReader::new(files)) }
    }
}

#[cfg(feature = "glob")]
mod glob_input {
    use std::fs::{File, metadata};
    use std::vec::IntoIter;

    extern crate glob;
    use self::glob::glob;

    impl super::Input<super::BMReader<IntoIter<File>>> {
        pub fn glob<P: Iterator>(patterns: P) -> super::Input<super::BMReader<IntoIter<File>>>
            where P::Item: AsRef<str>
        {
            let mut files = Vec::new();
            for pattern in patterns {
                match glob(pattern.as_ref()) {
                    Ok(entries) => {
                        for entry in entries {
                            match entry {
                                Ok(ref path) => {
                                    match metadata(path) {
                                        Ok(m) => {
                                            if m.is_dir() {
                                                warn!("Pattern matches to a directory: {}",
                                                      path.display());
                                                continue;
                                            }
                                        }
                                        Err(err) => {
                                            warn!("Cannot read file metadata {}\n{}",
                                                  path.display(),
                                                  err);
                                            continue;
                                        }
                                    }
                                    match File::open(path) {
                                        Ok(file) => files.push(file),
                                        Err(err) => {
                                            warn!("Cannot open file {}\n{}", path.display(), err)
                                        }
                                    }
                                }
                                Err(err) => warn!("Cannot access file {}", err),
                            }
                        }
                    }
                    Err(err) => warn!("Bad pattern {}\n{}", pattern.as_ref(), err),
                }
            }

            super::Input::files(files.into_iter())
        }
    }
}

#[cfg(feature = "glob")]
pub use self::glob_input::*;
