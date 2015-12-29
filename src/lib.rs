#![feature(alloc, heap_api)]

extern crate alloc;

pub mod reader;

pub use reader::{BitReader,BigEndian,LittleEndian};

mod util {
    use std;
    use std::{fmt, error, convert, io};

    // Allocates a byte buffer with word alignment
    pub unsafe fn allocate_buffer(size: usize) -> *mut u8 {
        use alloc::heap::allocate;
        debug_assert!(size % word_bytes() == 0);
        allocate(size, std::mem::align_of::<usize>())
    }

    pub unsafe fn deallocate_buffer(buf: *mut u8, size: usize) {
        use alloc::heap::deallocate;
        deallocate(buf, size, std::mem::align_of::<usize>());
    }

    #[inline(always)]
    pub fn word_bytes() -> usize {
        std::mem::size_of::<usize>()
    }
    #[inline(always)]
    pub fn word_bits() -> usize {
        std::mem::size_of::<usize>() * 8
    }

    pub const DEFAULT_BUFFER_SIZE : u32 = 4096; // 4KB buffer
    pub const MIN_AVAILABLE : usize = 4; // Minimum number of words in the buffer at any given time

    pub type Result<T> = std::result::Result<T, Error>;

    #[derive(Debug)]
    pub enum Error {
        UnexpectedEOF,
        Io(io::Error)
    }

    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                Error::UnexpectedEOF => f.write_str("Unexpected End-Of-File"),
                Error::Io(ref io) => fmt::Display::fmt(io, f)
            }
        }
    }

    impl convert::From<io::Error> for Error {
        fn from(io: io::Error) -> Error {
            Error::Io(io)
        }
    }

    impl error::Error for Error {
        fn description(&self) -> &str {
            match *self {
                Error::UnexpectedEOF => "Unexpected End-Of-File",
                Error::Io(ref io) => io.description()
            }
        }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::Io(ref io) => Some(io),
            _ => None
        }
    }
}


}
