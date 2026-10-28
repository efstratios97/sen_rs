use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use regex::Regex;

const CLASS_REGEX: &str = r"class\s+(?<class>\w+)";
const MEMBER_NAME_REGEX: &str = r"\bvar\s+(?<name>[A-Za-z_][A-Za-z0-9_]*)\s*:";
const MEMBER_STL_TYPE_REGEX: &str = r":\s*(?<type>[A-Za-z_][A-Za-z0-9_.]*)\s*(?:\[|;|$)";
const MEMBER_VISIBILITY_REGEX: &str = r"\[\s*[^\]]*\b(?<visibility>writable)\b";
const MEMBER_NETWORK_PROTOCOL_REGEX: &str = r"\[\s*[^\]]*\b(?<protocol>confirmed)\b";

pub struct StlCode {
    classes: HashMap<String, GeneratedTrait>,
}

struct GeneratedTrait {
    pub name: String,
    pub member_variables
}

struct MemberVariable {
    pub visibility: String,
    pub name: String,
    pub stl_type: String,
    pub network_protocol: Protocol,
}

enum Protocol {
    UDP,
    TCP,
}

use thiserror::Error;

#[derive(Error, Debug)]
pub enum StlParserError {
    #[error("Could nor convert STL File in String")]
    StlFileConversion,
}

impl StlCode {
    pub fn parse_stl_code(&mut self, mut buffer: BufReader<File>) {
        let code = StlCode::extract_code(self, buffer, None);
    }

    pub fn extract_code(
        &mut self,
        mut buffer: BufReader<File>,
        mut current_class: Option<String>,
    ) -> Result<(), StlParserError> {
        let mut current_line = String::new();

        let line_read = buffer.read_line(&mut current_line);

        if line_read.is_err() {
            return Ok(());
        }

        if let Some(class_name) = Self::get_class(&current_line) {
            self.classes.insert(
                class_name.clone(),
                GeneratedTrait {
                    name: class_name.clone(),
                },
            );
            current_class = Some(class_name);
        }

        if let Some(member_name) = Self::get_class_member_name(&current_line) {
            self.classes.insert(
                current_class.unwrap().clone(),
                GeneratedTrait {
                    name: member_name.clone(),
                    MemberVariable
                },
            );
        }

        let _ = self.extract_code(buffer, current_class);

        Ok(())
    }

    fn get_class(stl_line: &str) -> Option<String> {
        let class_name = Regex::new(CLASS_REGEX).unwrap().captures(stl_line)?;
        Some(class_name["class"].to_string())
    }

    fn get_class_member_name(stl_line: &str) -> Option<String> {
        let class_name = Regex::new(CLASS_REGEX).unwrap().captures(stl_line)?;
        Some(class_name["class"].to_string())
    }
}
