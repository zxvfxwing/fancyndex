/*
*
* Override of FromSegements trait on std::path::PathBuf
* https://github.com/SergioBenitez/Rocket/blob/master/lib/src/request/param.rs
* -> line 328
*
* We need to accept hidden files (".dotfiles") in given routes, which Rocket doesn't allow.
*
* Workaround to override in Rust :
* Wrap the wanted object into an other one.
*
* Special thanks to Rocket's matrix channel
* Message:
* https://matrix.to/#/!NkeCEOxYAAanlPMmEf:matrix.org/$15106551905wxJcZ:spokonline.net
*
*/

use std::path::PathBuf;
use rocket::http::uri::{URI, Segments, SegmentError};
use rocket::request::FromSegments;
use std::fmt::Debug;

pub struct UnsafePBuf {
    path: PathBuf
}

impl UnsafePBuf {
    pub fn new() -> UnsafePBuf {
        UnsafePBuf {
            path: PathBuf::new()
        }
    }

    pub fn push(&mut self, suffix: &str) {
        self.path.push(suffix);
    }

    pub fn pop(&mut self) {
        self.path.pop();
    }

    pub fn path(&self) -> PathBuf {
        self.path.to_path_buf()
    }
}

impl<'a> FromSegments<'a> for UnsafePBuf {
    type Error = SegmentError;

    fn from_segments(segments: Segments<'a>) -> Result<UnsafePBuf, SegmentError> {
        let mut unsafe_p = UnsafePBuf::new();

        for segment in segments {
            let decoded = URI::percent_decode(segment.as_bytes())
                .map_err(|e| SegmentError::Utf8(e))?;

            if decoded == ".." {
                unsafe_p.pop();
            }
            else if decoded.starts_with('*') {
                return Err(SegmentError::BadStart('*'))
            } else if decoded.ends_with(':') {
                return Err(SegmentError::BadEnd(':'))
            } else if decoded.ends_with('>') {
                return Err(SegmentError::BadEnd('>'))
            } else if decoded.ends_with('<') {
                return Err(SegmentError::BadEnd('<'))
            } else if decoded.contains('/') {
                return Err(SegmentError::BadChar('/'))
            } else if cfg!(windows) && decoded.contains('\\') {
                return Err(SegmentError::BadChar('\\'))
            } else {
                unsafe_p.push(&*decoded)
            }
        }
        
        Ok(unsafe_p)
    }
}
