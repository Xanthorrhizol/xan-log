use log::{Level, LevelFilter, Log, Metadata, Record, SetLoggerError};

#[allow(dead_code)]
static DEFAULT_BG_RED_TEXT: &str = "\x1b[31m";
#[allow(dead_code)]
static DEFAULT_BG_GREEN_TEXT: &str = "\x1b[32m";
#[allow(dead_code)]
static DEFAULT_BG_YELLOW_TEXT: &str = "\x1b[33m";
#[allow(dead_code)]
static DEFAULT_BG_BLUE_TEXT: &str = "\x1b[34m";
static DEFAULT_BG_GRAY_TEXT: &str = "\x1b[90m";

static RED_BG_BLACK_TEXT: &str = "\x1b[41;30m";
static YELLOW_BG_BLACK_TEXT: &str = "\x1b[43;30m";
static BLUE_BG_WHITE_TEXT: &str = "\x1b[44;37m";
static GREEN_BG_BLACK_TEXT: &str = "\x1b[42;30m";
static CYAN_BG_BLACK_TEXT: &str = "\x1b[46;30m";
static DEFAULT_BG_DEFAULT_TEXT: &str = "\x1b[49;39m";

pub extern crate log;

#[cfg(not(feature = "tracing"))]
static LOGGER: XanLogger = XanLogger {
    log_level: LevelFilter::Off,
};

pub struct XanLogger {
    log_level: LevelFilter,
}

impl XanLogger {
    pub fn new(log_level: LevelFilter) -> Self {
        Self { log_level }
    }
}

impl Log for XanLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.log_level
    }

    fn log(&self, r: &Record) {
        let metadata = r.metadata();
        let args = r.args();
        let module_path = r.module_path();
        let file = r.file();
        let line = r.line();
        let kv = r.key_values();
        println!(
            "[{}] [{}@{}:{}] [target:{}] [module_path:{}] {}",
            chrono::Utc::now()
                .format("%Y-%m-%dT%H:%M:%S.%3fZ")
                .to_string(),
            if let Some(lv) = kv.get("level".into()) {
                format!(
                    "{}{}{}",
                    CYAN_BG_BLACK_TEXT,
                    lv.to_string().to_uppercase(),
                    DEFAULT_BG_DEFAULT_TEXT
                )
            } else {
                match metadata.level() {
                    Level::Error => {
                        format!("{}ERROR{}", RED_BG_BLACK_TEXT, DEFAULT_BG_DEFAULT_TEXT)
                    }
                    Level::Warn => {
                        format!("{}WARN{}", YELLOW_BG_BLACK_TEXT, DEFAULT_BG_DEFAULT_TEXT)
                    }
                    Level::Info => format!("{}INFO{}", BLUE_BG_WHITE_TEXT, DEFAULT_BG_DEFAULT_TEXT),
                    Level::Debug => {
                        format!("{}DEBUG{}", GREEN_BG_BLACK_TEXT, DEFAULT_BG_DEFAULT_TEXT)
                    }
                    Level::Trace => {
                        format!("{}TRACE{}", DEFAULT_BG_GRAY_TEXT, DEFAULT_BG_DEFAULT_TEXT)
                    }
                }
            },
            file.unwrap_or(""),
            line.unwrap_or(0),
            metadata.target().to_string(),
            module_path.unwrap_or(""),
            args,
        );
    }

    fn flush(&self) {}
}

#[cfg(not(feature = "tracing"))]
pub fn init_logger() -> Result<(), SetLoggerError> {
    let log_level = std::env::var("LOG_LEVEL").unwrap_or("off".to_string());
    let log_level = match log_level.to_lowercase().as_str() {
        "trace" => LevelFilter::Trace,
        "debug" => LevelFilter::Debug,
        "info" => LevelFilter::Info,
        "warn" => LevelFilter::Warn,
        "error" => LevelFilter::Error,
        _ => LevelFilter::Off,
    };
    log::set_logger(&LOGGER).map(|()| log::set_max_level(log_level))
}

#[cfg(feature = "tracing")]
pub fn init_logger() -> Result<(), SetLoggerError> {
    let log_level = std::env::var("LOG_LEVEL").unwrap_or("off".to_string());

    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new(log_level));

    let _ = tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .try_init();

    Ok(())
}

#[test]
fn test() {
    unsafe {
        std::env::set_var("LOG_LEVEL", "INFO");
    }
    let _ = init_logger();
    log::error!("This is an error message");
    log::warn!("This is a warning message");
    log::info!("This is an info message");
    log::debug!("This is a debug message");
    log::trace!("This is a trace message");
    log::log!(target: "my_target", Level::Info, level = "CUSTOM"; "test log {}", "TEST");
}
