use chrono::Local;
use tracing::Level;
use tracing::Subscriber;
use tracing_subscriber::fmt;
use tracing_subscriber::registry::LookupSpan;

// Reexport tracing macros and structs
pub use tracing::instrument;
pub use tracing::instrument::Instrument;
pub use tracing::{
    debug, debug_span, error, error_span, info, info_span, trace, trace_span, warn, warn_span,
};

// Reeport the whole crate
extern crate tracing;
pub mod tr {
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

/// ## Header Info
/// Tracing info log variant with colored header.
/// ```
/// # use util_logs::*;
/// hinfo!(BLUE, "CONFIG", "Could not open {}", "config.txt")
/// ```
#[macro_export]
macro_rules! hinfo {
    ($color:expr, $label:expr, $fmt:expr $(, $arg:expr)*) => {
        {
            let message = format!($fmt $(, $arg)*);
            let mut iter = message.split('\n').collect::<Vec<&str>>().into_iter();
            if let Some(line) = iter.next() {
                tr::info!("{}{BOLD}{:>12}:{RESET} {}", $color, format!("[{}]", $label), line);
            }
            for line in iter {
                tr::info!("{:>13} {}", "", line);
            }
        }
    };
}

/// ## Header Warning
/// Tracing warn log variant with colored header.
/// ```
/// # use util_logs::*;
/// hwarn!(YELLOW, "HTTP", "Unable to ping {}", "http://foo.bar")
/// ```
#[macro_export]
macro_rules! hwarn {
    ($color:expr, $label:expr, $fmt:expr $(, $arg:expr)*) => {
        {
            let message = format!($fmt $(, $arg)*);
            let mut iter = message.split('\n').collect::<Vec<&str>>().into_iter();
            if let Some(line) = iter.next() {
                tr::warn!("{}{BOLD}{:>12}:{RESET} {}", $color, format!("[{}]", $label), line);
            }
            for line in iter {
                tr::warn!("{:>13} {}", "", line);
            }
        }
    };
}

/// ## Header Error
/// Tracing error log variant with colored header.
/// ```
/// # use util_logs::*;
/// herror!(RED, "CRASH", "Application panicked!")
/// ```
#[macro_export]
macro_rules! herror {
    ($color:expr, $label:expr, $fmt:expr $(, $arg:expr)*) => {
        {
            let message = format!($fmt $(, $arg)*);
            let mut iter = message.split('\n').collect::<Vec<&str>>().into_iter();
            if let Some(line) = iter.next() {
                tr::error!("{}{BOLD}{:>12}:{RESET} {}", $color, format!("[{}]", $label), line);
            }
            for line in iter {
                tr::error!("{:>13} {}", "", line);
            }
        }
    };
}

/// ## Header Info Extended
/// Tracing info log variant with colored header. Text can be colored too.
/// ```
/// # use util_logs::*;
/// hinfo_ext!(BLUE, "CONFIG", BLUE, "Could not open {}", "config.txt")
/// ```
#[macro_export]
macro_rules! hinfo_ext {
    ($color1:expr, $label:expr, $color2:expr, $fmt:expr $(, $arg:expr)*) => {
        {
            let message = format!($fmt $(, $arg)*);
            let mut iter = message.split('\n').collect::<Vec<&str>>().into_iter();
            if let Some(line) = iter.next() {
                tr::info!("{}{BOLD}{:>12}:{RESET} {}{}{RESET}", $color1, format!("[{}]", $label), $color2, line);
            }
            for line in iter {
                tr::info!("{:>13} {}{}{RESET}", "", $color2, line);
            }
        }
    };
}

/// ## Header Warning Extended
/// Tracing warn log variant with colored header. Text can be colored too.
/// ```
/// # use util_logs::*;
/// hwarn_ext!(YELLOW, "HTTP", YELLOW, "Unable to ping {}", "http://foo.bar")
/// ```
#[macro_export]
macro_rules! hwarn_ext {
    ($color1:expr, $label:expr, $color2:expr, $fmt:expr $(, $arg:expr)*) => {
        {
            let message = format!($fmt $(, $arg)*);
            let mut iter = message.split('\n').collect::<Vec<&str>>().into_iter();
            if let Some(line) = iter.next() {
                tr::warn!("{}{BOLD}{:>12}:{RESET} {}{}{RESET}", $color1, format!("[{}]", $label), $color2, line);
            }
            for line in iter {
                tr::warn!("{:>13} {}{}{RESET}", "", $color2, line);
            }
        }
    };
}

/// ## Header Error Extended
/// Tracing error log variant with colored header. Text can be colored too.
/// ```
/// # use util_logs::*;
/// herror_ext!(RED, "CRASH", RED, "Application panicked!")
/// ```
#[macro_export]
macro_rules! herror_ext {
    ($color1:expr, $label:expr, $color2:expr, $fmt:expr $(, $arg:expr)*) => {
        {
            let message = format!($fmt $(, $arg)*);
            let mut iter = message.split('\n').collect::<Vec<&str>>().into_iter();
            if let Some(line) = iter.next() {
                tr::error!("{}{BOLD}{:>12}:{RESET} {}{}{RESET}", $color1, format!("[{}]", $label), $color2, line);
            }
            for line in iter {
                tr::error!("{:>13} {}{}{RESET}", "", $color2, line);
            }
        }
    };
}

/// ## Info Colored
/// Tracing info log variant with colored text.
/// ```
/// # use util_logs::*;
/// cinfo!(BLUE, "Could not open {}", "config.txt")
/// ```
#[macro_export]
macro_rules! cinfo {
    ($color2:expr, $fmt:expr $(, $arg:expr)*) => {
        {
            let message = format!($fmt $(, $arg)*);
            let mut iter = message.split('\n').collect::<Vec<&str>>().into_iter();
            for line in iter {
                tr::info!("{}{}{RESET}", $color2, line);
            }
        }
    };
}

/// ## Warning Colored
/// Tracing warn log variant with colored text.
/// ```
/// # use util_logs::*;
/// cwarn!(YELLOW, "Unable to ping {}", "http://foo.bar")
/// ```
#[macro_export]
macro_rules! cwarn {
    ($color2:expr, $fmt:expr $(, $arg:expr)*) => {
        {
            let message = format!($fmt $(, $arg)*);
            let mut iter = message.split('\n').collect::<Vec<&str>>().into_iter();
            for line in iter {
                tr::warn!("{}{}{RESET}", $color2, line);
            }
        }
    };
}

/// ## Error Colored
/// Tracing error log variant with colored text.
/// ```
/// # use util_logs::*;
/// cerror!(RED, "Application panicked!")
/// ```
#[macro_export]
macro_rules! cerror {
    ($color2:expr, $fmt:expr $(, $arg:expr)*) => {
        {
            let message = format!($fmt $(, $arg)*);
            let mut iter = message.split('\n').collect::<Vec<&str>>().into_iter();
            for line in iter {
                tr::error!("{}{}{RESET}", $color2, line);
            }
        }
    };
}

// #=========================#
// #=== TRACING FORMATTER ===#

use tracing::level_filters::LevelFilter;
use tracing_subscriber::{
    layer::SubscriberExt,
    util::{SubscriberInitExt, TryInitError},
};

/// Initialize tracing subscriber.
pub fn tracing_init() {
    try_tracing_init().unwrap();
}

/// Try to initialize tracing subscriber.
pub fn try_tracing_init() -> Result<(), TryInitError> {
    // Create the formatted logging layer
    let fmt_layer = tracing_subscriber::fmt::layer().event_format(TracingFormatter);

    // Create the tracing registry
    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(LevelFilter::INFO)
        .try_init()
}

pub struct TracingFormatter;
impl<S, N> fmt::FormatEvent<S, N> for TracingFormatter
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> fmt::FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        ctx: &fmt::FmtContext<'_, S, N>,
        mut writer: fmt::format::Writer<'_>,
        event: &tracing::Event<'_>,
    ) -> std::fmt::Result {
        // Format values from the event's's metadata:
        let metadata = event.metadata();

        // Write the timestamp
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
        write!(&mut writer, "{DIM}{timestamp}{RESET} ")?;

        // Write the event level
        match *metadata.level() {
            Level::INFO => write!(&mut writer, "{GREEN}{:>5}{RESET} ", metadata.level()),
            Level::WARN => write!(&mut writer, "{YELLOW}{:>5}{RESET} ", metadata.level()),
            Level::ERROR => write!(&mut writer, "{RED}{:>5}{RESET} ", metadata.level()),
            _ => write!(&mut writer, "{} ", metadata.level()),
        }?;

        write!(writer, "{DIM}>> ")?;
        if let Some(scope) = ctx.event_scope() {
            let mut iter = scope.from_root().peekable();
            while let Some(sp) = iter.next() {
                if iter.peek().is_some() {
                    write!(writer, "{} > ", sp.name())?;
                } else {
                    write!(writer, "{}", sp.name())?;
                }
            }
        }
        write!(writer, " ⣿ {RESET}")?;

        // Write fields on the event
        ctx.field_format().format_fields(writer.by_ref(), event)?;
        writeln!(writer)
    }
}
