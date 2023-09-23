use core::fmt;
use std::io;

/// Writes everything successfully to the endless void of nothingness ...
/// and counts written bytes and calls to `write`!
///
/// # Example
/// ```
/// use std::io::{Cursor, copy};
/// use test_utils::streams::VoidWriter;
///
/// let mut reader = Cursor::new(vec![0u8; 100]);
/// let mut writer = VoidWriter::new();
/// let read = copy(&mut reader, &mut writer).unwrap();
/// assert_eq!(read, 100);
/// assert_eq!(writer.wrote(), 100);
/// ```
#[derive(Default)]
pub struct VoidWriter {
    wrote: usize,
    calls: usize,
}

impl VoidWriter {
    /// Create a new instance of [`VoidWriter`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the amount of written bytes to the
    /// [`VoidWriter`].
    pub fn wrote(&self) -> usize {
        self.wrote
    }

    /// Returns the amount of calls to `write` to the
    /// [`VoidWriter`].
    pub fn calls(&self) -> usize {
        self.calls
    }
}

impl io::Write for VoidWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.wrote += buf.len();
        self.calls += 1;
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl fmt::Display for VoidWriter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Wrote {} bytes in {} calls", self.wrote, self.calls)
    }
}
