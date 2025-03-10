pub struct UtilBufWrapper<const N: usize> {
    pub buffer: [u8; N],
    pub total: usize,
}

impl<const N: usize> UtilBufWrapper<N> {
    pub const fn new(buffer: [u8; N], total: usize) -> Self {
        Self { buffer, total }
    }

    pub const fn buffer(&self) -> &[u8] {
        self.buffer.split_at(self.total).0
    }

    pub const fn as_str(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(self.buffer()) }
    }
}

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
