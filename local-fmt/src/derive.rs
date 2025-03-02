use crate::ConstMessage;

pub struct CheckConstMessageArg<const M: usize>;

impl<const M: usize> CheckConstMessageArg<M> {
    pub const fn check<const N: usize>(
        message: &'static str,
        arg: ConstMessage<N>,
    ) -> ConstMessage<M> {
        if N == M {
            unsafe { std::mem::transmute::<ConstMessage<N>, ConstMessage<M>>(arg) }
        } else {
            panic!("{}", message);
        }
    }
}

#[macro_export]
macro_rules! check_const_message_arg {
    ($lang:expr, $value:expr, $count:literal, $($arg:tt)*) => {
        CheckConstMessageArg::check::<$count>(concat!("mismatch count of arguments in ",$lang," ",$value," message"),gen_const_message!($($arg)*))
    };
}
