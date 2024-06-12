use nom;

pub fn n<N: std::str::FromStr>(input: &str) -> nom::IResult<&str, N> {
    nom::combinator::map_res(nom::character::complete::digit1, N::from_str)(input)
}

#[macro_export]
macro_rules! impl_from_str_from_nom_parser {
    ($fn:ident, $obj:ident) => {
        impl std::str::FromStr for $obj {
            type Err = nom::error::Error<String>;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                use nom::Finish;
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

#[macro_export]
macro_rules! impl_from_str_for_obj_with_lieftimes_from_nom_parser {
    ($fn:ident, $obj:ident) => {
        impl<'input, 'output> TryFrom<&'input str> for $obj<'output>
        where
            'input: 'output,
        {
            type Error = nom::error::Error<String>;

            fn try_from(value: &'input str) -> Result<Self, Self::Error> {
                use nom::Finish;
                match $fn(value).finish() {
                    Ok((_remaining, object)) => Ok(object),
                    Err(nom::error::Error { input, code }) => Err(Self::Error {
                        input: input.to_string(),
                        code,
                    }),
                }
            }
        }
    };
}
