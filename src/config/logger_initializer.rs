
use colored::{Color, Colorize};

fn parse_color(env_var: &str, default: Color) -> Color {
    match std::env::var(env_var).unwrap_or_default().to_lowercase().as_str() {
        "red"     => Color::Red,
        "green"   => Color::Green,
        "yellow"  => Color::Yellow,
        "blue"    => Color::Blue,
        "magenta" => Color::Magenta,
        "cyan"    => Color::Cyan,
        "white"   => Color::White,
        _         => default,
    }
}

pub fn setup_logger() -> Result<(), fern::InitError> {
    colored::control::set_override(true);

    let color_info  = parse_color("LOG_COLOR_INFO",  Color::Green);
    let color_warn  = parse_color("LOG_COLOR_WARN",  Color::Yellow);
    let color_error = parse_color("LOG_COLOR_ERROR", Color::Red);

    fern::Dispatch::new()
        .format(move |out, message, record| {
            let formatted = format!(
            "{} | {} | {}",
            chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
            record.level(),
            message
        );

        let colored_line = match record.level() {
            log::Level::Error => formatted.color(color_error),
            log::Level::Warn  => formatted.color(color_warn),
            log::Level::Info  => formatted.color(color_info),
            log::Level::Debug => formatted.blue(),
            log::Level::Trace => formatted.dimmed(),
        };

        out.finish(format_args!("{}", colored_line))
        })
        .level(log::LevelFilter::Info)
        .chain(std::io::stdout())
        .chain(
            fern::Dispatch::new()
                .format(|out, message, record| {
                    out.finish(format_args!(
                        "{} | {} | {}",
                        chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
                        record.level(),
                        message
                    ))
                })
                .chain(fern::log_file("app_issues.log")?)
        )
        .apply()?;

    Ok(())
}