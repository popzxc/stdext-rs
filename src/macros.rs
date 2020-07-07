/// `compile_warning` macro is a brother of [`std::compile_error`],
/// which emits a compile-time warning with a provided message.
///
/// This implemented through an existing `dead_code` warning, thus the
/// output for the following example:
///
/// ```rust
/// # use stdext::compile_warning;
/// compile_warning!("Sample user-defined warning!");
/// ```
///
/// may look as follows:
///
/// ```text
/// warning: constant item is never used: `WARNING`
///   --> src/lib.rs:7:9
///   |
/// 7 |         const WARNING: &str = $expr;
///   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
/// ...
/// 11 | compile_warning!("Sample user-defined warning!");
///    | ------------------------------------------------- in this macro invocation
/// ```
///
/// Once [`proc_macro_diagnostics`] feature is stabilized, this macro will be replaced
/// with a proper proc-macro-based implementation.
///
/// This macro is intended to be used in the development process, as an alternative to the
/// [`unimplemented`] macro which doesn't cause code to panic.
///
/// [`std::compile_error`]: https://doc.rust-lang.org/std/macro.compile_error.html
/// [`proc_macro_diagnostics`]: https://github.com/rust-lang/rust/issues/54140
/// [`unimplemented`]: https://doc.rust-lang.org/std/macro.unimplemented.html
#[macro_export]
macro_rules! compile_warning {
    ($expr:expr) => {
        #[warn(dead_code)]
        const WARNING: &str = $expr;
    };
}

/// This macro returns the name of the enclosing function.
/// As the internal implementation is based on the [`std::any::type_name`], this macro derives
/// all the limitations of this function.
///
/// # Examples
///
/// ```rust
/// mod bar {
///     pub fn sample_function() {
///         use stdext::function_name;
///         assert!(function_name!().ends_with("bar::sample_function"));
///     }
/// }
///
/// bar::sample_function();
/// ```
///
/// [`std::any::type_name`]: https://doc.rust-lang.org/std/any/fn.type_name.html
#[macro_export]
macro_rules! function_name {
    () => {{
        // Okay, this is ugly, I get it. However, this is the best we can get on a stable rust.
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        // `3` is the length of the `::f`.
        &name[..name.len() - 3]
    }};
}
