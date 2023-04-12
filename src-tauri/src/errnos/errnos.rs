use super::prop::{PropErrno, PropErrnoParams};
use log::error;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
// use shared::log::log;
use std::{error::Error as StdError, fmt, io};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Errno {
    code: String,
    fixable: bool,
    params: Value,
}

impl fmt::Display for Errno {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

impl StdError for Errno {}

impl Errno {
    pub fn params(&self) -> Value {
        self.params.clone()
    }

    pub fn code(&self) -> String {
        self.code.clone()
    }

    pub fn is_fixable(&self) -> bool {
        self.fixable
    }

    /// this function merges the basic io::Error and the Errno
    /// # NOTE: only use this if the error cannot be converted to an Errno
    /// it will return only return the Errno
    /// also it will log the error for diagnostic purposes
    /// # Arguments
    /// * `err` - io::Error
    /// * `Errno` - closest Errno if any
    /// # Returns
    /// * `Errno`
    pub fn from_io(_err: io::Error, closest_errno: Option<Errno>) -> Self {
        let errno = closest_errno.unwrap_or_default();
        error!("io error : {}", _err);
        errno
    }

    /// Allows for translation of PropErrno to Errno
    /// # Arguments
    /// * `prop_errno` - PropErrnoParams - all the parameters needed for the error
    /// while all of them won't be used in all cases, they are all there for completeness
    /// # Returns
    /// * `Errno`
    pub fn from_prop_errno(prop_errno: PropErrno, params: &mut PropErrnoParams) -> Self {
        // warn!(
        //     "Src: {}, Dst: {}, Err: {}",
        //     params.src(),
        //     params.dst(),
        //     prop_errno
        // );
        match prop_errno {
            PropErrno::Unknown => Errno::unknown(),
            PropErrno::NoMem => Errno::mem(),
            PropErrno::NoStorage => Errno::storage(),
            PropErrno::PlatformNotSupported => Errno::unsupported_platform(),
            PropErrno::TooManyTasks => Errno::too_many_tasks(),
            PropErrno::ExpectedDir => Errno::expected_dir(params.src()),
            PropErrno::ExpectedDirVal(val) => Errno::expected_dir(val),
            PropErrno::ExpectedFileVal(val) => Errno::expected_file(val),
            PropErrno::ExpectedFile => Errno::expected_file(params.src()),
            PropErrno::ExpectedDstDir => Errno::expected_dir(params.dst()),
            PropErrno::ExpectedDstFile => Errno::expected_file(params.dst()),
            PropErrno::Read => Errno::read(params.src()),
            PropErrno::ReadVal(val) => Errno::read(val),
            PropErrno::ReadDir => Errno::read_dir(params.src()),
            PropErrno::ReadDirVal(val) => Errno::read_dir(val),
            PropErrno::Write => Errno::write(params.dst()),
            PropErrno::WriteVal(val) => Errno::write(val),
            PropErrno::PathNotFound => Errno::path_not_found(params.src()),
            PropErrno::PathNotFoundVal(val) => Errno::path_not_found(val),
            PropErrno::Copy => Errno::copy(params.src(), params.dst()),
            PropErrno::CopyVal(src, dst) => Errno::copy(src, dst),
            PropErrno::Move => Errno::cut(params.src(), params.dst()),
            PropErrno::MoveVal(src, dst) => Errno::cut(src, dst),
            PropErrno::Delete => Errno::delete(params.src()),
            PropErrno::DeleteVal(val) => Errno::delete(val),
            PropErrno::Rename => Errno::rename(params.src()),
            PropErrno::RenameVal(val) => Errno::rename(val),
            PropErrno::Finish => Errno::finish(params.dst()),
            PropErrno::FinishVal(val) => Errno::finish(val),
            PropErrno::ReadPerm => Errno::read_perm(params.src()),
            PropErrno::ReadPermVal(val) => Errno::read_perm(val),
            PropErrno::WritePerm => Errno::write_perm(params.dst()),
            PropErrno::WritePermVal(val) => Errno::write_perm(val),
            PropErrno::ReadWritePerm => Errno::read_write_perm(params.src()),
            PropErrno::ReadWritePermVal(val) => Errno::read_write_perm(val),
            PropErrno::SetPerm => Errno::perm_set(params.dst()),
            PropErrno::SetPermVal(val) => Errno::perm_set(val),
            PropErrno::EntityCreation(val) => Errno::entity_create(val),
            PropErrno::SetMeta => Errno::meta_set(params.dst()),
            PropErrno::SetMetaVal(val) => Errno::meta_set(val),
            PropErrno::GetMeta => Errno::meta_fetch(params.src()),
            PropErrno::GetMetaVal(val) => Errno::meta_fetch(val),
            PropErrno::BrokenSymlink => Errno::broken_sym_link(params.src()),
            PropErrno::BrokenSymlinkVal(val) => Errno::broken_sym_link(val),
            PropErrno::CorruptedFile => Errno::corrupted_file(params.src()),
            PropErrno::CorruptedFileVal(val) => Errno::corrupted_file(val),
            PropErrno::CorruptedHeaderVal(corrupted_file) => {
                Errno::corrupted_header(params.src(), corrupted_file)
            }
            PropErrno::Compress => Errno::compress(params.src()),
            PropErrno::CompressVal(val) => Errno::compress(val),
            PropErrno::Decompress => Errno::decompress(params.src()),
            PropErrno::DecompressVal(val) => Errno::decompress(val),
            PropErrno::Encrypt => Errno::encrypt(params.src()),
            PropErrno::EncryptVal(val) => Errno::encrypt(val),
            PropErrno::Decrypt => Errno::decrypt(params.src()),
            PropErrno::DecryptVal(val) => Errno::decrypt(val),
            PropErrno::PasswordLength(min, max) => Errno::password_len(params.src(), min, max),
            PropErrno::PasswordLengthVal(val, min, max) => Errno::password_len(val, min, max),
            PropErrno::InvalidPassword => Errno::password_incorrect(params.src()),
            PropErrno::PasswordInterpolation => Errno::password_interp(params.src()),
            PropErrno::PasswordInterpolationVal(val) => Errno::password_interp(val),
            PropErrno::InvalidPasswordVal(val) => Errno::password_incorrect(val),
            PropErrno::InvalidPassOrCorrupt => {
                Errno::password_incorrect_or_corrupted_file(params.src())
            }
            PropErrno::InvalidPassOrCorruptVal(val) => {
                Errno::password_incorrect_or_corrupted_file(val)
            }
            PropErrno::Unpack => Errno::unpack(params.src()),
            PropErrno::UnpackVal(val) => Errno::unpack(val),
            PropErrno::UnpackOutofDir => Errno::unpack_out_dst(params.src()),
            PropErrno::UnpackOutofDirVal(val) => Errno::unpack_out_dst(val),
            PropErrno::PathCopy => Errno::path_copy(params.src()),
            PropErrno::PathCopyVal(val) => Errno::path_copy(val),
            PropErrno::Interrupted => Errno::interrupted(params.src()),
            PropErrno::InterruptedVal(val) => Errno::interrupted(val),
            PropErrno::PathNormalize => Errno::path_normalize(params.src()),
            PropErrno::PathNormalizeVal(val) => Errno::path_normalize(val),
            PropErrno::Loop => Errno::loops(params.src()),
            PropErrno::LoopVal(val) => Errno::loops(val),
            _ => Errno::unknown(),
        }
    }
}

/// implmenet the default errno
impl Default for Errno {
    fn default() -> Self {
        Errno::unknown()
    }
}

impl Errno {
    pub fn unknown() -> Self {
        Self {
            fixable: false,
            code: "unknown_err".to_string(),
            params: json!({}),
        }
    }

    pub fn mem() -> Self {
        Self {
            fixable: false,
            code: "mem_err".to_string(),
            params: json!({}),
        }
    }

    pub fn unsupported_platform() -> Self {
        Self {
            fixable: false,
            code: "unsupported_platform_err".to_string(),
            params: json!({}),
        }
    }

    pub fn too_many_tasks() -> Self {
        Self {
            fixable: false,
            code: "too_many_tasks_err".to_string(),
            params: json!({}),
        }
    }

    pub fn storage() -> Self {
        Self {
            fixable: false,
            code: "store_err".to_string(),
            params: json!({}),
        }
    }

    pub fn interrupted(base: String) -> Self {
        Self {
            fixable: false,
            code: "interrupted_err".to_string(),
            params: json!({ "base": base }),
        }
    }

    pub fn read_dir(base: String) -> Self {
        Self {
            fixable: false,
            code: "read_dir_err".to_string(),
            params: json!({ "base": base }),
        }
    }

    pub fn rename(base: String) -> Self {
        Self {
            fixable: false,
            code: "rename_err".to_string(),
            params: json!({ "base": base }),
        }
    }

    pub fn copy(from: String, to: String) -> Self {
        Self {
            fixable: false,
            code: "copy_err".to_string(),
            params: json!({ "from": from, "to": to }),
        }
    }

    pub fn cut(from: String, to: String) -> Self {
        Self {
            fixable: false,
            code: "cut_err".to_string(),
            params: json!({ "from": from, "to": to }),
        }
    }

    pub fn read(base: String) -> Self {
        Self {
            fixable: false,
            code: "read_err".to_string(),
            params: json!({ "base": base }),
        }
    }

    pub fn loops(base: String) -> Self {
        Self {
            fixable: false,
            code: "loop_err".to_string(),
            params: json!({ "base": base }),
        }
    }

    pub fn write(base: String) -> Self {
        Self {
            fixable: false,
            code: "write_err".to_string(),
            params: json!({ "base": base }),
        }
    }
    pub fn delete(base: String) -> Self {
        Self {
            fixable: false,
            code: "delete_err".to_string(),
            params: json!({ "base": base }),
        }
    }

    pub fn finish(base: String) -> Self {
        Self {
            fixable: false,
            code: "finish_err".to_string(),
            params: json!({ "base": base }),
        }
    }

    pub fn expected_file(base: String) -> Self {
        Self {
            fixable: true,
            code: "expected_file_err".to_string(),
            params: json!({
            "base": base,
            }),
        }
    }

    pub fn expected_dir(base: String) -> Self {
        Self {
            fixable: true,
            code: "expected_dir_err".to_string(),
            params: json!({
            "base": base,
            }),
        }
    }

    pub fn write_perm(base: String) -> Self {
        Self {
            fixable: false,
            code: "write_perm_err".to_string(),
            params: json!({ "base": base }),
        }
    }
    pub fn read_perm(base: String) -> Self {
        Self {
            fixable: false,
            code: "read_perm_err".to_string(),
            params: json!({ "base": base }),
        }
    }
    pub fn read_write_perm(base: String) -> Self {
        Self {
            fixable: false,
            code: "read_write_perm_err".to_string(),
            params: json!({ "base": base }),
        }
    }
    pub fn perm_set(base: String) -> Self {
        Self {
            fixable: false,
            code: "perm_set_err".to_string(),
            params: json!({ "base": base }),
        }
    }
    pub fn path_not_found(base: String) -> Self {
        Self {
            fixable: true,
            code: "path_not_found_err".to_string(),
            params: json!({ "base": base }),
        }
    }

    pub fn path_copy(base: String) -> Self {
        Self {
            fixable: false,
            code: "path_copy_err".to_string(),
            params: json!({ "base": base }),
        }
    }

    pub fn path_normalize(base: String) -> Self {
        Self {
            fixable: false,
            code: "path_normalize_err".to_string(),
            params: json!({ "base": base }),
        }
    }

    pub fn entity_create(base: String) -> Self {
        Self {
            fixable: false,
            code: "entity_create_err".to_string(),
            params: json!({ "base": base }),
        }
    }
    pub fn broken_sym_link(base: String) -> Self {
        Self {
            fixable: false,
            code: "broken_sym_link_err".to_string(),
            params: json!({ "base": base }),
        }
    }
    pub fn meta_set(base: String) -> Self {
        Self {
            fixable: false,
            code: "meta_set_err".to_string(),
            params: json!({ "base": base }),
        }
    }
    pub fn meta_fetch(base: String) -> Self {
        Self {
            fixable: false,
            code: "meta_fetch_err".to_string(),
            params: json!({ "base": base }),
        }
    }
    pub fn corrupted_file(base: String) -> Self {
        Self {
            fixable: false,
            code: "corrupted_file_err".to_string(),
            params: json!({ "base": base }),
        }
    }
    pub fn compress(base: String) -> Self {
        Self {
            fixable: false,
            code: "compress_err".to_string(),
            params: json!({ "base": base }),
        }
    }
    pub fn decompress(base: String) -> Self {
        Self {
            fixable: false,
            code: "decompress_err".to_string(),
            params: json!({ "base": base }),
        }
    }
    pub fn corrupted_header(base: String, path: String) -> Self {
        Self {
            fixable: false,
            code: "corrupted_header_err".to_string(),
            params: json!({
                "base": base,
                "path": path,
            }),
        }
    }
    pub fn unpack(base: String) -> Self {
        Self {
            fixable: false,
            code: "unpack_err".to_string(),
            params: json!({ "base": base }),
        }
    }
    pub fn unpack_out_dst(base: String) -> Self {
        Self {
            fixable: false,
            code: "unpack_out_dst_err".to_string(),
            params: json!({ "base": base }),
        }
    }
    pub fn encrypt(base: String) -> Self {
        Self {
            fixable: false,
            code: "encrypt_err".to_string(),
            params: json!({ "base": base }),
        }
    }
    pub fn decrypt(base: String) -> Self {
        Self {
            fixable: false,
            code: "decrypt_err".to_string(),
            params: json!({ "base": base }),
        }
    }
    pub fn password_len(base: String, min: u8, max: u8) -> Self {
        Self {
            fixable: true,
            code: "password_len_err".to_string(),
            params: json!({
                "base": base,
            "min": min,
            "max": max
            }),
        }
    }
    pub fn password_interp(base: String) -> Self {
        Self {
            fixable: true,
            code: "password_interp_err".to_string(),
            params: json!({ "base": base }),
        }
    }
    pub fn password_incorrect(base: String) -> Self {
        Self {
            fixable: true,
            code: "password_incorrect_err".to_string(),
            params: json!({ "base": base }),
        }
    }
    pub fn password_incorrect_or_corrupted_file(base: String) -> Self {
        Self {
            fixable: true,
            code: "password_incorrect_or_corrupted_file_err".to_string(),
            params: json!({ "base": base }),
        }
    }
}
