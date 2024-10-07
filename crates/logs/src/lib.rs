use std::fmt::Write;

use tracing::Level;
use tracing::Subscriber;
use tracing_subscriber::fmt;
use tracing_subscriber::registry::LookupSpan;
use indicatif::{ProgressState, ProgressStyle};

pub mod trc {
    pub use tracing::*;
}

// #=======================#
// #=== COLOR CONSTANTS ===#

pub const RESET: &str = "\x1B[0m";
pub const BOLD: &str = "\x1B[1m";
pub const DIM: &str = "\x1B[2m";
pub const ITALIC: &str = "\x1B[3m";
pub const UNDERLINE: &str = "\x1B[4m";
pub const REVERSED: &str = "\x1B[7m";

pub const BLACK: &str = "\x1B[30m";
pub const RED: &str = "\x1B[31m";
pub const GREEN: &str = "\x1B[32m";
pub const YELLOW: &str = "\x1B[33m";
pub const BLUE: &str = "\x1B[34m";
pub const MAGENTA: &str = "\x1B[35m";
pub const CYAN: &str = "\x1B[36m";
pub const WHITE: &str = "\x1B[37m";

pub const BG_BLACK: &str = "\x1B[40m";
pub const BG_RED: &str = "\x1B[41m";
pub const BG_GREEN: &str = "\x1B[42m";
pub const BG_YELLOW: &str = "\x1B[43m";
pub const BG_BLUE: &str = "\x1B[44m";
pub const BG_MAGENTA: &str = "\x1B[45m";
pub const BG_CYAN: &str = "\x1B[46m";
pub const BG_WHITE: &str = "\x1B[47m";


// #======================#
// #=== LOGGING MACROS ===#

/// Tracing info log with colored header.
/// ```
/// # use crate::info;
/// info!(BLUE, "CONFIG", "Could not open {}", "config.txt")
/// ```
#[macro_export]
macro_rules! info {
    ($color:expr, $label:expr, $fmt:expr $(, $arg:expr)*) => {
        {
            let message = format!($fmt $(, $arg)*);
            let mut iter = message.split('\n').collect::<Vec<&str>>().into_iter();
            if let Some(line) = iter.next() {
                trc::info!("{}{BOLD}{:>12}:{RESET} {}", $color, format!("[{}]", $label), line);
            }
            for line in iter {
                trc::info!("{}{BOLD}{:>13}{RESET} {}", $color, "", line);
            }
        }
    };
}

/// Tracing warning log with colored header.
/// ```
/// # use crate::warn;
/// warn!(YELLOW, "HTTP", "Unable to ping {}", "http://foo.bar")
/// ```
#[macro_export]
macro_rules! warn {
    ($color:expr, $label:expr, $fmt:expr $(, $arg:expr)*) => {
        {
            let message = format!($fmt $(, $arg)*);
            let mut iter = message.split('\n').collect::<Vec<&str>>().into_iter();
            if let Some(line) = iter.next() {
                trc::warn!("{}{BOLD}{:>12}:{RESET} {}", $color, format!("[{}]", $label), line);
            }
            for line in iter {
                trc::warn!("{}{BOLD}{:>13}{RESET} {}", $color, "", line);
            }
        }
    };
}

/// Tracing error log with colored header.
/// ```
/// # use crate::error;
/// error!(RED, "CRASH", "Application panicked!")
/// ```
#[macro_export]
macro_rules! error {
    ($color:expr, $label:expr, $fmt:expr $(, $arg:expr)*) => {
        {
            let message = format!($fmt $(, $arg)*);
            let mut iter = message.split('\n').collect::<Vec<&str>>().into_iter();
            if let Some(line) = iter.next() {
                trc::error!("{}{BOLD}{:>12}:{RESET} {}", $color, format!("[{}]", $label), line);
            }
            for line in iter {
                trc::error!("{}{BOLD}{:>13}{RESET} {}", $color, "", line);
            }
        }
    };
}

/// Tracing info log with colored header. Text can be colored too.
/// ```
/// # use crate::info_ext;
/// info_ext!(BLUE, "CONFIG", BLUE, "Could not open {}", "config.txt")
/// ```
#[macro_export]
macro_rules! info_ext {
    ($color1:expr, $label:expr, $color2:expr, $fmt:expr $(, $arg:expr)*) => {
        {
            let message = format!($fmt $(, $arg)*);
            let mut iter = message.split('\n').collect::<Vec<&str>>().into_iter();
            if let Some(line) = iter.next() {
                trc::info!("{}{BOLD}{:>12}:{RESET} {}{}{RESET}", $color1, format!("[{}]", $label), $color2, line);
            }
            for line in iter {
                trc::info!("{}{BOLD}{:>13}{RESET} {}{}{RESET}", $color1, "", $color2, line);
            }
        }
    };
}

/// Tracing info log with colored header. Text can be colored too.
/// ```
/// # use crate::warn_ext;
/// warn_ext!(YELLOW, "HTTP", YELLOW, "Unable to ping {}", "http://foo.bar")
/// ```
#[macro_export]
macro_rules! warn_ext {
    ($color1:expr, $label:expr, $color2:expr, $fmt:expr $(, $arg:expr)*) => {
        {
            let message = format!($fmt $(, $arg)*);
            let mut iter = message.split('\n').collect::<Vec<&str>>().into_iter();
            if let Some(line) = iter.next() {
                trc::warn!("{}{BOLD}{:>12}:{RESET} {}{}{RESET}", $color1, format!("[{}]", $label), $color2, line);
            }
            for line in iter {
                trc::warn!("{}{BOLD}{:>13}{RESET} {}{}{RESET}", $color1, "", $color2, line);
            }
        }
    };
}

/// Tracing info log with colored header. Text can be colored too.
/// ```
/// # use crate::error_ext;
/// error_ext!(RED, "CRASH", RED, "Application panicked!")
/// ```
#[macro_export]
macro_rules! error_ext {
    ($color1:expr, $label:expr, $color2:expr, $fmt:expr $(, $arg:expr)*) => {
        {
            let message = format!($fmt $(, $arg)*);
            let mut iter = message.split('\n').collect::<Vec<&str>>().into_iter();
            if let Some(line) = iter.next() {
                trc::error!("{}{BOLD}{:>12}:{RESET} {}{}{RESET}", $color1, format!("[{}]", $label), $color2, line);
            }
            for line in iter {
                trc::error!("{}{BOLD}{:>13}{RESET} {}{}{RESET}", $color1, "", $color2, line);
            }
        }
    };
}


// #=======================#
// #=== SPECIAL LOGGING ===#

/// Log HTTP StatusCode and colorise the output based on code
#[cfg(feature = "reqwest")]
pub fn log_status(status: reqwest::StatusCode, body: &String) {
    if !status.is_success() {
        let text = status.canonical_reason().unwrap_or_default();
        warn!(RED, format!("HTTP {}", status.as_u16()), "{text}\n{body}");
    }
}


// #=====================#
// #=== PROGRESS BARS ===#

/// Return the style of iteration like progress bar
pub fn empty_bar() -> ProgressStyle {
    ProgressStyle::with_template("").unwrap()
}

/// Return the style of iteration like progress bar
pub fn progress_bar() -> ProgressStyle {
    ProgressStyle::with_template("\n {spinner:.green} [{elapsed_precise:.bold.cyan}]: [{wide_bar:.cyan/blue}] {human_pos}/{human_len} ({eta} left)\n\n\n").unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
        .progress_chars("#>-")
}

/// Return the style of a download like progress bar
pub fn download_bar() -> ProgressStyle {
    ProgressStyle::with_template("\n {spinner:.green} [{elapsed_precise:.bold.cyan}]: [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta} left)\n\n\n").unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
        .progress_chars("#>-")
}


// #=========================#
// #=== TRACING FORMATTER ===#

use tracing::level_filters::LevelFilter;
use tracing_indicatif::IndicatifLayer;
use tracing_subscriber::{layer::SubscriberExt, util::{TryInitError, SubscriberInitExt}};

/// Initialize tracing subscriber.
pub fn tracing_init() {
    try_tracing_init().unwrap();
}

/// Try to initialize tracing subscriber.
pub fn try_tracing_init() -> Result<(), TryInitError> {
    // Create the IndicatifLayer
    let indicatif_layer = IndicatifLayer::new();

    // Create the formatted logging layer
    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_writer(indicatif_layer.get_stderr_writer())
        .event_format(TracingFormatter);

    // Create the tracing registry
    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(indicatif_layer)
        .with(LevelFilter::INFO)
        .try_init()
}

pub struct TracingFormatter;
impl<S, N> fmt::FormatEvent<S, N> for TracingFormatter where S: Subscriber + for<'a> LookupSpan<'a>, N: for<'a> fmt::FormatFields<'a> + 'static {
    fn format_event(&self, ctx: &fmt::FmtContext<'_, S, N>, mut writer: fmt::format::Writer<'_>, event: &tracing::Event<'_>) -> std::fmt::Result {
        // Format values from the event's's metadata:
        let metadata = event.metadata();

        // Write the event level
        match *metadata.level() {
            Level::INFO => write!(&mut writer, "{GREEN}{:>5}{RESET} | ", metadata.level()),
            Level::WARN => write!(&mut writer, "{YELLOW}{:>5}{RESET} | ", metadata.level()),
            Level::ERROR => write!(&mut writer, "{RED}{:>5}{RESET} | ", metadata.level()),
            _ => write!(&mut writer, "{} | ", metadata.level()),
        }?;

        // Count number of spans
        let mut spans = 0;
        if let Some(scope) = ctx.event_scope() {
            spans = scope.from_root().count();
        }

        // Add the span column
        if spans <= 3 {
            write!(writer, "{:<3} ⣿⣿ ", ">".repeat(spans))?;
        } else {
            write!(writer, ">>{} ⣿⣿ ", spans - 2)?;
        }

        // Write fields on the event
        ctx.field_format().format_fields(writer.by_ref(), event)?;
        writeln!(writer)
    }
}
