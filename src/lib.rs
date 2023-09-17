use log::{LevelFilter, Log, Metadata, Record, SetLoggerError};

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

    fn log(&self, record: &Record) {
        println!("{}", XanRecordJson::from(record));
    }

    fn flush(&self) {}
}

struct XanRecordJson(String);

impl<'a> From<&Record<'a>> for XanRecordJson {
    fn from(r: &Record<'a>) -> XanRecordJson {
        let metadata = r.metadata();
        let args = r.args();
        let module_path = r.module_path();
        let file = r.file();
        let line = r.line();
        XanRecordJson(
            serde_json::json!({
                "level": metadata.level().to_string(),
                "target": metadata.target().to_string(),
                "module_path": module_path.unwrap_or(""),
                "file": file.unwrap_or(""),
                "line": line.unwrap_or(0),
                "message": args,
            })
            .to_string(),
        )
    }
}

impl std::fmt::Display for XanRecordJson {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_str(&self.0)
    }
}

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

#[test]
fn test() {
    std::env::set_var("LOG_LEVEL", "info");
    init_logger().unwrap();
    log::info!("hello {}", "world");
    log::debug!("hello {}", "world");
}
