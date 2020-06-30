/// `compile_warning` macro is a brother of [`std::compile_error`],
/// which emits a compile-time warning with a provided message.
///
/// This implemented through an existing `dead_code` warning, thus the
/// output for the following example:
///
/// ```rust,ignore
/// compile_warning!("Sample user-defined warning!");
/// ```
///
/// may look as follows:
///
/// ```text
/// warning: constant item is never used: `WARNING`
///   --> src/lib.rs:7:9
///   |
/// 7  |         const WARNING: &str = $expr;
///   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
/// ...
/// 11 | compile_warning!("Sample user-defined warning!");
///   | ------------------------------------------------- in this macro invocation
/// ```
///
/// Once [`proc_macro_diagnostics`] feature is stabilized, this macro will be replaced
/// with a proper proc-macro-based implementation.
///
/// [`std::compile_error`]: https://doc.rust-lang.org/std/macro.compile_error.html
/// [`proc_macro_diagnostics`]: https://github.com/rust-lang/rust/issues/54140
#[macro_export]
macro_rules! compile_warning {
    ($expr:expr) => {
        #[warn(dead_code)]
        const WARNING: &str = $expr;
    };
}
