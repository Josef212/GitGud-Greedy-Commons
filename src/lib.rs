pub mod dtos;

use std::str::FromStr;
use std::io::Write;
use chrono::Utc;
use env_logger::fmt::Color;
use log::LevelFilter;

pub fn init_logger(log_level: &String) {
    env_logger::Builder::new()
        .format(|buf, record| {
            let level = record.level();
            let mut style = buf.style();
            style
                .set_bold(true)
                .set_color(match level {
                    log::Level::Error => Color::Red,
                    log::Level::Warn => Color::Yellow,
                    log::Level::Trace => Color::Cyan,
                    _ => Color::White
                });

            writeln!(buf,
                     "({}) [{}]: {}",
                     Utc::now().format("%Y-%m-%d %H:%M:%S"),
                     style.value(level),
                     record.args()
            )
        })
        .filter_level(LevelFilter::from_str(log_level.as_str()).unwrap_or(LevelFilter::Error))
        // .is_test(true)
        .init();
    std::panic::set_hook(Box::new(|err| {log::error!("{}", err)}));
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
