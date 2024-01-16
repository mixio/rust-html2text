#[cfg(feature = "html_trace_bt")]
extern crate backtrace;

/* This is to work around a false positive for the clippy warning
 * `match_on_same_arms`.
 * See https://github.com/Manishearth/rust-clippy/issues/1390
 */
#[cfg(not(feature = "html_trace"))]
#[inline(always)]
pub fn nop() {}

#[cfg(feature = "html_trace")]
#[macro_export]
#[doc(hidden)]
macro_rules! html_trace {
    ($fmt:expr) => {
         #[cfg(feature = "html_trace_bt")]
         {
             let bt = ::backtrace::Backtrace::new();
             log::info!( concat!($fmt, " at {:?}"), bt );
         }
         #[cfg(not(feature = "html_trace_bt"))]
         {
             log::info!($fmt);
         }
    };
    ($fmt:expr, $( $args:expr ),*) => {
         #[cfg(feature = "html_trace_bt")]
         {
             let bt = ::backtrace::Backtrace::new();
             log::info!( concat!($fmt, " at {:?}"), $( $args ),* , bt );
         }
         #[cfg(not(feature = "html_trace_bt"))]
         {
             log::info!($fmt, $( $args ),*);
         }
    };
}
#[cfg(not(feature = "html_trace"))]
#[macro_export]
#[doc(hidden)]
macro_rules! html_trace {
    ($fmt:expr) => {
        $crate::macros::nop();
    };
    ($fmt:expr, $( $args:expr ),*) => {
        $crate::macros::nop();
    };
}

#[cfg(feature = "html_trace")]
#[macro_export]
#[doc(hidden)]
macro_rules! html_trace_quiet {
    ($fmt:expr) => {
         log::trace!( $fmt );
    };
    ($fmt:expr, $( $args:expr ),*) => {
         log::trace!( $fmt, $( $args ),* );
    };
}

#[cfg(not(feature = "html_trace"))]
#[macro_export]
#[doc(hidden)]
macro_rules! html_trace_quiet {
    ($fmt:expr) => {
        $crate::macros::nop();
    };
    ($fmt:expr, $( $args:expr ),*) => {
        $crate::macros::nop();
    };
}
