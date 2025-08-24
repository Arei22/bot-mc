use env_logger::{Builder, Env};
use std::io::Write;

pub fn init() {
    let env: Env = Env::default().filter_or(
        "MY_LOG_LEVEL",
        "info,serenity=off,serenity::client=off,serenity::http=off,tracing=off",
    );

    Builder::from_env(env)
        .format(|buf, record| {
            let level_style = buf.default_level_style(record.level());

            writeln!(
                buf,
                "{level_style}[{}] [{}] [{}] {}{level_style:#}",
                record.level(),
                record.target(),
                chrono::Local::now().format("%d/%m/%Y - %H:%M:%S"),
                record.args()
            )
        })
        .init();
}
