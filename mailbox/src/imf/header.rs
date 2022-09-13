/// Represents a header in an Internet Message Format message
pub struct ImfHeader {
    name: HeaderName,
    body: HeaderBody
}

pub enum HeaderName {
    Subject,
    To,
    From,
    //TODO
}

pub enum HeaderBody {
    Unstructured(String),
    
}