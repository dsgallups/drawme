#[macro_export]
macro_rules! impl_from {
    ($source:ty, $enum:ty, ($param:ident) => $body:block ) => {
        impl From<$source> for $enum {
            fn from($param: $source) -> $enum {
                $body
            }
        }

        impl<'a> From<$source> for std::borrow::Cow<'a, $enum> {
            fn from($param: $source) -> std::borrow::Cow<'a, $enum> {
                std::borrow::Cow::Owned($body)
            }
        }
    };
}
