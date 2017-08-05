use padding::PaddingRequest;
use rocket::http::RawStr;

#[derive(FromForm)]
pub struct PaddingRequestUrl<'a> {
    pub str: &'a RawStr,
    pub len: usize,
    pub ch: Option<String>,
}


#[derive(Debug)]
pub enum ParamsError {
    BadLength,
    LengthTooLong,
}

impl<'a> ::std::convert::From<ParamsError> for &'a str {
    fn from(err: ParamsError) -> &'a str {
        match err {
            ParamsError::BadLength => "illegal content length",
            ParamsError::LengthTooLong => "illegal padding length",
        }
    }
}

pub fn read_params(url: &PaddingRequestUrl) -> Result<PaddingRequest, ParamsError> {
    if url.str.len() < 1 {
        return Err(ParamsError::BadLength);
    }

    if url.len > 999 {
        return Err(ParamsError::LengthTooLong);
    }

    Ok(PaddingRequest::new(
        url.len,
        url.str.to_string(),
        match url.ch {
            Some(ref ch) => ch.chars().nth(0).unwrap_or(' '),
            None => ' ',
        },
    ))
}
