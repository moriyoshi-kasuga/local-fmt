/// A wrapper for a byte array buffer.
/// The total field is the number of bytes used in the buffer.
/// The buffer field is the byte array.
/// The buffer field is split at the total field to get the used bytes.
pub struct UtilBufWrapper<const N: usize> {
    pub buffer: [u8; N],
    pub total: usize,
}

impl<const N: usize> UtilBufWrapper<N> {
    pub const fn new(buffer: [u8; N], total: usize) -> Self {
        Self { buffer, total }
    }

    /// Returns the buffer split at the total field.
    /// The used bytes are returned.
    pub const fn buffer(&self) -> &[u8] {
        self.buffer.split_at(self.total).0
    }

    /// Returns the buffer as a str.
    /// 
    /// # Safety
    /// This function assumes that the buffer contains valid UTF-8 bytes.
    /// This is safe because:
    /// 1. The buffer is initialized with ASCII digits and '-' character only
    /// 2. All operations that modify the buffer (const_u128_to_str, const_i128_to_str)
    ///    only write ASCII characters (0-9, '-')
    /// 3. ASCII is a subset of UTF-8, so ASCII bytes are always valid UTF-8
    pub const fn as_str(&self) -> &str {
        // SAFETY: Buffer only contains ASCII digits and '-' character,
        // which are always valid UTF-8
        unsafe { std::str::from_utf8_unchecked(self.buffer()) }
    }
}

/// Converts a u128 to a str as a byte array.
/// The buffer is 39 bytes long.
/// The maximum number of digits in a u128 is 39.
///
/// # Example
/// ```
/// use local_fmt::utils::const_u128_to_str;
///
/// const BUFFER: &[u8] = const_u128_to_str(1234567890).buffer();
/// assert_eq!(BUFFER, b"1234567890");
/// ```
pub const fn const_u128_to_str(n: u128) -> UtilBufWrapper<39> {
    if n == 0 {
        let buf: [u8; 39] = [
            b'0', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        return UtilBufWrapper::new(buf, 1);
    }
    let mut buffer = [0u8; 39];
    let mut i = 0;
    let mut n = n;
    while n > 0 {
        buffer[i] = b'0' + (n % 10) as u8;
        n /= 10;
        i += 1;
    }
    let mut result = [0u8; 39];
    let mut j = 0;
    while j < i {
        result[j] = buffer[i - j - 1];
        j += 1;
    }
    UtilBufWrapper::new(result, i)
}

/// Converts an i128 to a str as a byte array.
/// The buffer is 40 bytes long.
/// The maximum number of digits in an i128 is 39.
/// The first byte is a '-' if the number is negative.
///
/// # Example
/// ```
/// use local_fmt::utils::const_i128_to_str;
///
/// const BUFFER: &[u8] = const_i128_to_str(-1234567890).buffer();
/// assert_eq!(BUFFER, b"-1234567890");
/// ```
pub const fn const_i128_to_str(n: i128) -> UtilBufWrapper<40> {
    let UtilBufWrapper { buffer: buf, total } = const_u128_to_str(n.unsigned_abs());
    let mut buffer = [0u8; 40];
    let mut i = 0;
    if n < 0 {
        buffer[i] = b'-';
        i += 1;
    }
    let mut j = 0;
    while j < total {
        buffer[i] = buf[j];
        i += 1;
        j += 1;
    }

    UtilBufWrapper::new(buffer, i)
}

/// A macro for creating a panic message with placeholders for arguments.
/// The message is formatted with the arguments and then a panic is raised.
///
/// This macro uses the same argument formatting as [`crate::fmt_builder`].
/// You can specify arguments as string literals, byte slices, or numbers
/// (both signed and unsigned) using the appropriate format specifiers.
///
/// # Example
/// ```rust, compile_fail
/// use local_fmt::{panic_builder, gen_static_message};
///
/// const _: () = {
///     let message = gen_static_message!("Error: {0} {1} {2}");
///     panic_builder!(message, ["This"], ["is"], ["a test"]);
/// };
/// ```
/// ```text
/// error[E0080]: evaluation of constant value failed
///  --> local-fmt/src/utils.rs:103:5
///   |
/// 9 |     panic_builder!(message, ["This"], ["is"], ["a test"]);
///   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'Error: This is a test', local-fmt/src/utils.rs:9:5
///   |
/// ```
#[macro_export]
macro_rules! panic_builder {
    ($message:ident, $([$($arg:tt)+]),* $(,)?) => {
        {
            let buffer = $crate::fmt_builder!($message,
                $(
                    [$($arg)+],
                )*
            );
            panic!("{}", buffer.as_str());
        }
    };
}

/// Formats a message with placeholders for arguments.
///
/// # Example
/// ```
/// use local_fmt::{fmt_builder, UtilBufWrapper, StaticMessage, gen_static_message};
///
/// const MESSAGE: StaticMessage<1> = gen_static_message!("Hello, {0}!");
///
/// // Example with string
/// {
///     const TEXT: &'static str = fmt_builder!(MESSAGE, ["World"]).as_str();
///
///     assert_eq!(TEXT, "Hello, World!");
/// }
///
/// // Example with signed number
/// {
///     const TEXT: UtilBufWrapper<1024> = fmt_builder!(MESSAGE, [i; -1234567890i32]);
///     assert_eq!(TEXT.as_str(), "Hello, -1234567890!");
/// }
///
/// // Example with unsigned number
/// {
///    const TEXT: UtilBufWrapper<1024> = fmt_builder!(MESSAGE, [u; 1234567890usize]);
///    assert_eq!(TEXT.as_str(), "Hello, 1234567890!");
/// }
///
/// // Example with bytes
/// {
///     const WORLD: &[u8] = b"World";
///     const TEXT: UtilBufWrapper<1024> = fmt_builder!(MESSAGE, [b; WORLD]);
///     assert_eq!(TEXT.as_str(), "Hello, World!");
/// }
///
/// // Example with multiple arguments
/// const BYTES: &[u8] = b"Bytes";
/// const THIS_MESSAGE: StaticMessage<4> = gen_static_message!("Hello, {0} {1} {2} {3}!");
/// const TEXT: UtilBufWrapper<1024> = fmt_builder!(THIS_MESSAGE, ["World"], [u; 1], [i; -2], [b; BYTES]);
/// assert_eq!(TEXT.as_str(), "Hello, World 1 -2 Bytes!");
#[macro_export]
macro_rules! fmt_builder {
    (@ $arg:literal) => {
        $arg.as_bytes()
    };
    (@ $arg:ident) => {
        $arg.as_bytes()
    };
    (@ b; $arg:ident) => {
        $arg
    };
    (@ u; $arg:expr ) => {
        $crate::const_u128_to_str($arg as u128).buffer()
    };
    (@ i; $arg:expr) => {
        $crate::const_i128_to_str($arg as i128).buffer()
    };
    ($message:ident, $([$($args:tt)+]),* $(,)?) => {
        unsafe {
            $message.const_format::<1024>(
                &[
                    $(
                        $crate::fmt_builder!(@ $($args)+),
                    )*
                ]
            )
        }
    };
}
