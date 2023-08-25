use std::io::Write;


pub trait Draw {
    type Error;
    fn draw<W: Write>(&self, writer: W) -> Result<(), Self::Error>;
}
