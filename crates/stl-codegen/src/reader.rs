// Copyright (c) 2026 Efstratios Pahis
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org>, at your option.
// This file may not be copied, modified, or distributed except
// according to those terms.

use std::{
    fs::File,
    io::{self, BufRead, BufReader, Read},
};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum StlReaderError {
    #[error("Could not read-in STL File")]
    StlFileIO(#[from] io::Error),
    #[error("Could nor convert STL File in String")]
    StlFileConversion,
}

pub struct StlReader {
    pub stl_file_path: String,
}

impl StlReader {
    pub fn read_stl_file(&self) -> Result<(), StlReaderError> {
        let mut file = File::open(&self.stl_file_path)?;

        let reader = BufReader::new(file);

        for line in reader.lines() {}

        Ok(())
    }
}
