use nom::error;

pub type ParserFn<'i, O, E = (&'i [u8], error::ErrorKind)> = fn(&[u8]) -> nom::IResult<&'i [u8], O, E>;

pub trait Parser<'i, I, O, E = (I, error::ErrorKind)>
where
    I: Into<&'i [u8]>,
{
    fn parse(&self, i: I) -> nom::IResult<I, O, E>;
}

impl<'i, O> Parser<'i, &'i [u8], O> for ParserFn<'i, O> 
{
    fn parse(&self, i: &'i [u8]) -> nom::IResult<&'i [u8], O> {
        self(i)
    } 
}
