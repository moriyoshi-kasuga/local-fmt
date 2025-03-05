use crate::ConstMessage;

pub struct CheckConstMessageArg<const M: usize, const N: usize>;

impl<const M: usize, const N: usize> CheckConstMessageArg<M, N> {
    pub const fn check(message: &'static str, arg: ConstMessage<N>) -> ConstMessage<M> {
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
        CheckConstMessageArg::check(
            concat!("Mismatch in the number of arguments for the message in language '", $lang, "' with key '", $key, "'."),
            $($arg)*
        )
    };
}
