use std::path::Path;

use crate::path::PathExt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PropErrnoParams {
    src: Option<String>, // source path
    dst: Option<String>, // the destination of the task
                         // task: Option<String>,               // the type of task running
}

impl Default for PropErrnoParams {
    fn default() -> Self {
        PropErrnoParams {
            // min_password_length: None,
            // max_password_length: None,
            src: None,
            dst: None,
            // task: None,
        }
    }
}

impl PropErrnoParams {
    pub fn new() -> Self {
        PropErrnoParams::default()
    }

    pub fn new_with_src_and_dst(src: String, dst: String) -> Self {
        Self {
            src: Some(src),
            dst: Some(dst),
        }
    }

    pub fn set_src(&mut self, src: String) -> &mut Self {
        self.src = Some(src);
        self
    }

    pub fn set_dst(&mut self, dst: String) -> &mut Self {
        self.dst = Some(dst);
        self
    }

    pub fn set_opt_dst(&mut self, dst: Option<String>) -> &mut Self {
        self.dst = dst;
        self
    }

    pub fn src(&mut self) -> String {
        if let Some(src) = self.src.take() {
            src
        } else {
            Path::unknown_path()
        }
    }

    pub fn dst(&mut self) -> String {
        if let Some(dst) = self.dst.take() {
            dst
        } else {
            Path::unknown_path()
        }
    }
}
