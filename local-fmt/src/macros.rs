// compiletime check macro
#[macro_export]
macro_rules! gen_const_message {
     (@gen $text:literal) => {
         $crate::MessageFormat::StaticText($text)
     };
     (@gen {$number:literal}) => {
         $crate::MessageFormat::Arg($number)
     };
     (@gen $ident:ident) => {
         $crate::MessageFormat::StaticText($ident)
     };
     (@gen $expr:expr) => {
         $crate::MessageFormat::StaticText($expr)
     };
     (unchecked, $arg_number:literal, $($tt:tt),*) => {
         $crate::ConstMessage::<$arg_number>::new_unchecked(vec![$(gen_const_message!(@gen $tt)),*])
     };
     ($arg_number:literal, $($tt:tt),* $(,)?) => {
        unsafe {
            $crate::ConstMessage::<$arg_number>::new_unchecked(
                const {
                    $crate::ConstMessage::<$arg_number>::const_check_and_panic(
                        &[$($crate::gen_const_message!(@gen $tt)),*]
                    )
                }
                .to_vec(),
            )
        }
     }
 }

// useable string macro
#[macro_export]
macro_rules! gen_message {
     (@gen $text:literal) => {
         $crate::MessageFormat::StaticText($text)
     };
     (@gen {$number:literal}) => {
         $crate::MessageFormat::Arg($number)
     };
     (@gen $ident:ident) => {
         $crate::MessageFormat::Text($ident)
     };
     (@gen $expr:expr) => {
         $crate::MessageFormat::Text($expr)
     };
     (unchecked, $arg_number:literal, $($tt:tt),* $(,)?) => {
         $crate::ConstMessage::<$arg_number>::new_unchecked(vec![$(gen_message!(@gen $tt)),*])
     };
     ($arg_number:literal, $($tt:tt),* $(,)?) => {
         $crate::ConstMessage::<$arg_number>::new(vec![$(gen_message!(@gen $tt)),*])
     }
 }
