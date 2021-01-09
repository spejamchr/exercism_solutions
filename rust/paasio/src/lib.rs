use std::io::{Read, Result, Write};

pub struct ReadStats<R: Read> {
    wrapped: R,
    reads: usize,
    bytes_through: usize,
}

impl<R: Read> ReadStats<R> {
    pub fn new(wrapped: R) -> Self {
        Self {
            wrapped,
            reads: 0,
            bytes_through: 0,
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.wrapped
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_through
    }

    pub fn reads(&self) -> usize {
        self.reads
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.reads += 1;
        self.wrapped.read(buf).map(|n| {
            self.bytes_through += n;
            n
        })
    }
}

pub struct WriteStats<W: Write> {
    wrapped: W,
    bytes_through: usize,
    writes: usize,
}

impl<W: Write> WriteStats<W> {
    pub fn new(wrapped: W) -> Self {
        Self {
            wrapped,
            bytes_through: 0,
            writes: 0,
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.wrapped
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_through
    }

    pub fn writes(&self) -> usize {
        self.writes
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.writes += 1;
        self.wrapped.write(buf).map(|n| {
            self.bytes_through += n;
            n
        })
    }

    fn flush(&mut self) -> Result<()> {
        self.wrapped.flush()
    }
}
