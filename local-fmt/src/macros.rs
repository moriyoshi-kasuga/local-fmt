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
    ($lang:expr, $key:expr, $count:literal, $($arg:tt)*) => {
        CheckConstMessageArg::check::<$count>(
            concat!("Mismatch in the number of arguments for the message in language '", $lang, "' with key '", $key, "'."),
            gen_const_message!($($arg)*)
        )
    };
}
