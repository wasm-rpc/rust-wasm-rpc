// Copied from https://raw.githubusercontent.com/DeMille/wasm-glue
// Thanks Sterling DeMille!
/// Sets stdout, stderr, and a custom panic hook
use pointer::Referenceable;
use std::boxed::Box;
use std::fmt;
use std::fmt::Write;
use std::io;
use std::string::String;

const LOG_LEVEL_WARNING: u32 = 3;
const LOG_LEVEL_INFO: u32 = 6;

extern "C" {
    fn __log_write(level: u32, message: *const u8);
}

fn _print(buf: &str) -> io::Result<()> {
    let string: String = buf.into();

    unsafe {
        __log_write(LOG_LEVEL_INFO, string.as_pointer());
    }

    Ok(())
}

fn _eprint(buf: &str) -> io::Result<()> {
    let string: String = buf.into();

    unsafe {
        __log_write(LOG_LEVEL_WARNING, string.as_pointer());
    }

    Ok(())
}

/// Used by the `print` macro
#[doc(hidden)]
pub fn _print_args(args: fmt::Arguments) {
    let mut buf = String::new();
    let _ = buf.write_fmt(args);
    let _ = _print(&buf);
}

/// Used by the `eprint` macro
#[doc(hidden)]
pub fn _eprint_args(args: fmt::Arguments) {
    let mut buf = String::new();
    let _ = buf.write_fmt(args);
    let _ = _eprint(&buf);
}

type PrintFn = fn(&str) -> io::Result<()>;

struct Printer {
    printfn: PrintFn,
    buffer: String,
    is_buffered: bool,
}

impl Printer {
    fn new(printfn: PrintFn, is_buffered: bool) -> Printer {
        Printer {
            buffer: String::new(),
            printfn,
            is_buffered,
        }
    }
}

impl io::Write for Printer {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.buffer.push_str(&String::from_utf8_lossy(buf));

        if !self.is_buffered {
            (self.printfn)(&self.buffer)?;
            self.buffer.clear();

            return Ok(buf.len());
        }

        if let Some(i) = self.buffer.rfind('\n') {
            let buffered = {
                let (first, last) = self.buffer.split_at(i);
                (self.printfn)(first)?;

                String::from(&last[1..])
            };

            self.buffer.clear();
            self.buffer.push_str(&buffered);
        }

        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        (self.printfn)(&self.buffer)?;
        self.buffer.clear();

        Ok(())
    }
}

/// Sets a line-buffered stdout, uses your JavaScript `print` function
pub fn set_stdout() {
    let printer = Printer::new(_print, true);
    io::set_print(Some(Box::new(printer)));
}

/// Sets a line-buffered stderr, uses your JavaScript `eprint` function
pub fn set_stderr() {
    let eprinter = Printer::new(_eprint, true);
    io::set_panic(Some(Box::new(eprinter)));
}

pub fn set_stdio() {
    set_stdout();
    set_stderr();
}
