use log::{LevelFilter, Metadata, Record, SetLoggerError};

struct LegacyLogger;

impl log::Log for LegacyLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        println!(
            "[{}] [{}] {}",
            record.target(),
            record.level(),
            record.args()
        )
    }

    fn flush(&self) {}
}

static LOGGER: LegacyLogger = LegacyLogger;

pub fn init(max_level: LevelFilter) -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER).map(|()| log::set_max_level(max_level))
}
