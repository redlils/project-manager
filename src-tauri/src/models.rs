use std::ffi::{OsStr, OsString};
use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;

pub struct Project {
    pub name: String,
    pub location: String,
    pub git_support: bool,
    pub has_remote: bool,
    pub origin_remote: Option<String>,
}

impl Serialize for Project {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        let mut s = serializer.serialize_struct("Project", 4)?;
        s.serialize_field("name", &self.name)?;
        s.serialize_field("location", &self.location)?;
        s.serialize_field("git_support", &self.git_support)?;
        s.serialize_field("has_remote", &self.has_remote)?;
        match &self.origin_remote {
            Some(url) => {s.serialize_field("origin_remote", &url)?;}
            None => {s.serialize_field("origin_remote", "")?;}
        }
        s.end()
    }
}