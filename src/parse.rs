#[macro_export]
macro_rules! impl_from_str_from_nom_parser {
    ($fn:ident, $obj:ident) => {
        use nom::Finish;

        impl std::str::FromStr for $obj {
            type Err = nom::error::Error<String>;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match $fn(s).finish() {
                    Ok((_remaining, object)) => Ok(object),
                    Err(nom::error::Error { input, code }) => Err(Self::Err {
                        input: input.to_string(),
                        code,
                    }),
                }
            }
        }
    };
}
