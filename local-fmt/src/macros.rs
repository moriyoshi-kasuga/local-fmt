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
     (unchecked, $($tt:tt),*) => {
         $crate::ConstMessage::Static(&[$(gen_const_message!(@gen $tt)),*])
     };
     ($($tt:tt),* $(,)?) => {
        const {$crate::ConstMessage::new_static(
            &[$($crate::gen_const_message!(@gen $tt)),*]
        )}
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
     (unchecked, $($tt:tt),* $(,)?) => {
         $crate::ConstMessage::Vec(vec![$(gen_message!(@gen $tt)),*])
     };
     ($($tt:tt),* $(,)?) => {
         $crate::ConstMessage::new(vec![$(gen_message!(@gen $tt)),*])
     }
 }
