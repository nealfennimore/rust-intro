#[derive(Debug)]
pub struct CoolNess(pub String, pub String, pub String);

#[derive(Debug)]
pub enum AddrSpace {
    CoolNess(CoolNess),
}