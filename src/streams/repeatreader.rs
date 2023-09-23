use std::io::{Cursor, Read, Result, Seek, SeekFrom};

/// Can be created with a given size and then repeats the
/// given content until finished reading.
pub struct RepeatReader<S> {
    size: usize,
    contents: S,
    read: usize,
}

impl<S> RepeatReader<S> {
    /// Create a new [`RepeatReader`] with the given `size`
    /// and `contents` to be read.
    ///
    /// # Example
    /// ```
    /// use test_utils::streams::RepeatReader;
    /// use std::io::Cursor;
    ///
    /// let contents = Cursor::new("hello world".bytes());
    /// let rr = RepeatReader::new(100, contents);
    /// ```
    pub fn new(size: usize, contents: S) -> Self {
        Self {
            contents,
            size,
            read: 0,
        }
    }

    /// Returns the number of bytes left to be read.
    pub fn left(&self) -> usize {
        self.size - self.read
    }
}

impl<S> RepeatReader<Cursor<S>> {
    /// Creates a new [`RepeatReader`] from the given slice
    /// implementation.
    ///
    /// # Example
    /// ```
    /// use test_utils::streams::RepeatReader;
    ///
    /// let v = vec![7u8; 20];
    /// RepeatReader::from_slice(100, v);
    ///
    /// let s = [7u8; 20];
    /// RepeatReader::from_slice(100, &s);
    /// ```
    pub fn from_slice(size: usize, slice: S) -> RepeatReader<Cursor<S>> {
        RepeatReader::new(size, Cursor::new(slice))
    }
}

impl RepeatReader<Cursor<&str>> {
    /// Creates a new [`RepeatReader`] from the given string.
    ///
    /// # Example
    /// ```
    /// use test_utils::streams::RepeatReader;
    ///
    /// RepeatReader::from_slice(100, "hello world");
    /// ```
    pub fn from_str(size: usize, str: &str) -> RepeatReader<Cursor<&str>> {
        RepeatReader::from_slice(size, str)
    }
}

impl<S> RepeatReader<S>
where
    S: Seek,
{
    /// Seeks the content buffer to `0` from start and sets
    /// the `read` state to `0`.
    pub fn reset(&mut self) -> Result<()> {
        self.contents.seek(SeekFrom::Start(0))?;
        self.read = 0;
        Ok(())
    }
}

impl<S> Read for RepeatReader<S>
where
    S: Read + Seek,
{
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let ln = if buf.len() > self.left() {
            self.left()
        } else {
            buf.len()
        };

        let mut read = self.contents.read(&mut buf[..ln])?;
        while read < ln {
            self.contents.seek(SeekFrom::Start(0))?;
            read += self.contents.read(&mut buf[read..ln])?;
        }

        self.read += ln;

        Ok(ln)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read() {
        let mut rr = RepeatReader::from_str(11, "hello world");
        let mut res = String::new();
        rr.read_to_string(&mut res).unwrap();
        assert_eq!(res, "hello world");

        let mut rr = RepeatReader::from_str(13, "hello world");
        let mut res = String::new();
        rr.read_to_string(&mut res).unwrap();
        assert_eq!(res, "hello worldhe");

        let mut rr = RepeatReader::from_str(3, "hello world");
        let mut res = String::new();
        rr.read_to_string(&mut res).unwrap();
        assert_eq!(res, "hel");

        let mut rr = RepeatReader::from_str(26, "hello world");
        let mut res = String::new();
        rr.read_to_string(&mut res).unwrap();
        assert_eq!(res, "hello worldhello worldhell");

        let mut rr = RepeatReader::from_str(0, "hello world");
        let mut res = String::new();
        rr.read_to_string(&mut res).unwrap();
        assert_eq!(res, "");
    }
}
