//! Internal error type for `proc_macro::Span`

use core::fmt;
use std::{
    borrow::{Borrow, Cow},
    ffi::OsStr,
    fmt::Display,
    fs,
    path::{Path, PathBuf},
};

use colored::Colorize;
use proc_macro::Span;
use quote::ToTokens;
use syn::spanned::Spanned;

/// The result type of a parser.
pub type Result<T> = std::result::Result<T, Error>;

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
