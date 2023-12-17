//! Internal error type for `proc_macro::Span`

use core::fmt;
use proc_macro::Span;
use quote::ToTokens;
use std::{borrow::Borrow, fmt::Display, path::PathBuf};
use syn::spanned::Spanned;

#[cfg(feature = "macro_error_debugging")]
mod macro_error_debugging_deps {
    pub use colored::Colorize;
    pub use std::{borrow::Cow, ffi::OsStr, fs, path::Path};
}

#[cfg(feature = "macro_error_debugging")]
use macro_error_debugging_deps::*;

/// The result type of a parser.
pub(crate) type Result<T> = std::result::Result<T, Error>;

#[allow(dead_code)]
#[derive(Debug)]
pub(crate) struct Error {
    filepath: PathBuf,
    message: String,
    span: Span,
    source_code: String,
}

impl Error {
    pub(crate) fn new<T>(err_tok: T, message: String) -> Self
    where
        T: ToTokens + Spanned + Borrow<T>,
    {
        #[cfg(feature = "macro_error_debugging")]
        {
            let file_path = err_tok.span().unwrap().source_file().path().clone();
            let source_code = fs::read_to_string(&file_path).unwrap();
            let span = err_tok.span().unwrap();

            Self {
                filepath: file_path,
                message: message,
                span: span,
                source_code: source_code,
            }
        }

        #[cfg(not(feature = "macro_error_debugging"))]
        {
            let span = err_tok.span().unwrap();
            Self {
                filepath: PathBuf::new(),
                message: message,
                span: span,
                source_code: "".to_string(),
            }
        }
    }

    #[cfg(feature = "macro_error_debugging")]
    fn render_location(
        formatter: &mut fmt::Formatter,
        message: &String,
        file_path: &Path,
        source_code: &str,
        span: &Span,
    ) -> fmt::Result {
        let start = span.start();
        let end = span.end();
        let mut end_column_pos = span.end().column();

        // if the span ranges multiple lines,
        // we gonna choose first line only.
        if start.line() != end.line() {
            // check the column count
            end_column_pos = source_code.lines().nth(start.line() - 1).unwrap().len();
        }

        let code_line = match start
            .line()
            .checked_sub(1)
            .and_then(|n| source_code.lines().nth(n))
        {
            Some(line) => line,
            None => "",
        };

        let filename = file_path
            .file_name()
            .map(OsStr::to_string_lossy)
            .unwrap_or(Cow::Borrowed("main.rs"));

        write!(
            formatter,
            "\n\
                 {error}{header}\n\
                 {indent}{arrow} {filename}:{linenum}:{colnum}\n\
                 {indent} {pipe}\n\
                 {label} {pipe} {code}\n\
                 {indent} {pipe} {offset}{underline} {message}\n\
                 ",
            error = "error".red().bold(),
            header = ": or-gen macro unable to parse file".bold(),
            indent = " ".repeat(start.line().to_string().len()),
            arrow = "-->".blue().bold(),
            filename = filename,
            linenum = start.line(),
            colnum = start.column(),
            pipe = "|".blue().bold(),
            label = start.line().to_string().blue().bold(),
            code = code_line.trim_end(),
            offset = " ".repeat(start.column()),
            underline = "^"
                .repeat(end_column_pos.saturating_sub(start.column()).max(1))
                .red()
                .bold(),
            message = message.to_string().red(),
        )
    }
}

#[cfg(feature = "macro_error_debugging")]
impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Self::render_location(
            f,
            &self.message,
            &self.filepath,
            &self.source_code,
            &self.span,
        )
    }
}

#[cfg(not(feature = "macro_error_debugging"))]
impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "
        {error}\n
        To see the more detailed cause of the error, You can use the `macro_error_debugging` feature in `or-rs-macros`.
        ", error = self.message)
    }
}
