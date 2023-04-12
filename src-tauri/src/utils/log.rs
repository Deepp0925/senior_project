/// Module for logging
use log::LevelFilter;
use log4rs::{
    append::{
        console::ConsoleAppender,
        rolling_file::{
            policy::compound::{roll, trigger, CompoundPolicy},
            RollingFileAppender,
        },
    },
    config::{Appender, Config, Logger, Root},
    encode::{json::JsonEncoder, pattern::PatternEncoder},
};

///Maximum size of a particular log file
const MAX_FILE_SIZE: u64 = 25 * 1024; // 25kb
///Maximum number of log files
const MAX_FILES: u32 = 25;

/// initalize loggin from a file
/// # Arguments
/// * `path` - the path to the log folder
/// # Returns
/// * `()` - nothing, there is no return value, it will panic if it fails
pub fn init_logger(path: Option<&str>) {
    let path = path.unwrap_or_else(|| "logs");
    let roll_files_path = format!("{}/log_{{}}.log", path);
    let file_path = format!("{}/log.log", path);

    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{d(%Y-%m-%d %H:%M:%S)} | {({l}):5.5} | {M} {f}:{L} — {m}{n}",
        )))
        .build();

    let policy = CompoundPolicy::new(
        Box::new(trigger::size::SizeTrigger::new(MAX_FILE_SIZE)),
        Box::new(
            roll::fixed_window::FixedWindowRoller::builder()
                .base(1)
                .build(&roll_files_path, MAX_FILES)
                .unwrap(),
        ),
    );

    let rolling_file = RollingFileAppender::builder()
        .encoder(Box::new(JsonEncoder::new()))
        .build(file_path, Box::new(policy))
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("rolling_file", Box::new(rolling_file)))
        .logger(
            Logger::builder()
                .appender("rolling_file")
                .build("rolling_files", LevelFilter::Trace),
        );

    let mut root = Root::builder();

    // TODO: check for a dev flag
    if true {
        root = root.appender("stdout");
    }

    let config = config
        .build(root.appender("rolling_file").build(LevelFilter::Trace))
        .unwrap();

    let _ = log4rs::init_config(config).unwrap();
}
