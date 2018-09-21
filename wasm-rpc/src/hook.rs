// Copied from https://raw.githubusercontent.com/DeMille/wasm-glue
// Thanks Sterling DeMille!
/// Sets stdout, stderr, and a custom panic hook
/// Sets stdout, stderr, and a custom panic hook

use pointer::Referenceable;
use std::string::String;
use std::boxed::Box;
use std::ffi::CString;
use std::os::raw::c_char;
use std::fmt;
use std::fmt::Write;
use std::panic;
use std::io;

const LOG_LEVEL_ERROR: u32 = 1;
const LOG_LEVEL_WARNING: u32 = 3;
const LOG_LEVEL_INFO: u32 = 6;

// these are the functions you'll need to privide with JS
extern {
    fn log_write(level: u32, message: *const u8);
    fn print(ptr: *const u8);
    fn eprint(ptr: *const u8);
    fn trace(ptr: *const u8);
}


fn _print(buf: &str) -> io::Result<()> {
    let string: String = buf.into();

    unsafe {
        log_write(LOG_LEVEL_INFO, string.as_pointer());
    }

    Ok(())
}

fn _eprint(buf: &str) -> io::Result<()> {
    let string: String = buf.into();

    unsafe {
        eprint(string.as_pointer());
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

/// Sets an unbuffered stdout, uses your JavaScript `print` function
pub fn set_stdout_unbuffered() {
    let printer = Printer::new(_print, false);
    io::set_print(Some(Box::new(printer)));
}

/// Sets a line-buffered stderr, uses your JavaScript `eprint` function
pub fn set_stderr() {
    let eprinter = Printer::new(_eprint, true);
    io::set_panic(Some(Box::new(eprinter)));
}

/// Sets an unbuffered stderr, uses your JavaScript `eprint` function
pub fn set_stderr_unbuffered() {
    let eprinter = Printer::new(_eprint, false);
    io::set_panic(Some(Box::new(eprinter)));
}

/// Sets a custom panic hook, uses your JavaScript `trace` function
pub fn set_panic_hook() {
    panic::set_hook(Box::new(|info| {
        let file = info.location().unwrap().file();
        let line = info.location().unwrap().line();
        let col = info.location().unwrap().column();

        let msg = match info.payload().downcast_ref::<&'static str>() {
            Some(s) => *s,
            None => {
                match info.payload().downcast_ref::<String>() {
                    Some(s) => &s[..],
                    None => "Box<Any>",
                }
            }
        };

        let err_info = format!("Panicked at '{}', {}:{}:{}", msg, file, line, col);
        let string: String = err_info.into();

        unsafe {
            log_write(LOG_LEVEL_ERROR, string.as_pointer());
        }
    }));
}

pub fn hook() {
    set_stdout();
    set_stderr();
    set_panic_hook();
}
