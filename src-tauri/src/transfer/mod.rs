// All implementation about the file transfer lives in this module
mod chunk;
mod dst_path;
// mod failed_part;
// mod file_assembler;
// mod file_compressor;
mod file_copier;
mod file_info;
// mod file_splitter;
mod header;
// mod part;
pub mod ffi;
mod parting_info;
mod settings;
mod status;
mod tracker;
mod transfer_manager;
mod worker;
