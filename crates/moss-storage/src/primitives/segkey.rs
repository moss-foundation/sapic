use anyhow::Result;
use redb::{Key, TypeName, Value};
use smallvec::SmallVec;
use std::ops::{Deref, DerefMut};

const SEGMENT_SEPARATOR: u8 = b':';
const SEGMENT_BUF_CAPACITY: usize = 128;

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct SegKeyBuf {
    inner: SmallVec<[u8; SEGMENT_BUF_CAPACITY]>,
}

impl Deref for SegKeyBuf {
    type Target = SmallVec<[u8; SEGMENT_BUF_CAPACITY]>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for SegKeyBuf {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<[u8]> for SegKeyBuf {
    fn as_ref(&self) -> &[u8] {
        &self.inner
    }
}

impl SegKeyBuf {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: SmallVec::with_capacity(capacity),
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.inner
    }
}

impl std::borrow::Borrow<[u8]> for SegKeyBuf {
    fn borrow(&self) -> &[u8] {
        &self.inner
    }
}

impl std::fmt::Display for SegKeyBuf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(&self.inner))
    }
}

impl Value for SegKeyBuf {
    type SelfType<'a>
        = SegKeyBuf
    where
        Self: 'a;

    type AsBytes<'a>
        = &'a [u8]
    where
        Self: 'a;

    fn fixed_width() -> Option<usize> {
        None
    }

    fn type_name() -> TypeName {
        TypeName::new("AnyKey")
    }

    fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> Self::AsBytes<'a> {
        &value.inner
    }

    fn from_bytes<'a>(data: &'a [u8]) -> Self::SelfType<'a>
    where
        Self: 'a,
    {
        SegKeyBuf {
            inner: data.to_vec().into(),
        }
    }
}

impl Key for SegKeyBuf {
    fn compare(data1: &[u8], data2: &[u8]) -> std::cmp::Ordering {
        data1.cmp(data2)
    }
}

pub struct SegKey<'a> {
    inner: &'a [u8],
}

impl<'a> AsRef<[u8]> for SegKey<'a> {
    fn as_ref(&self) -> &[u8] {
        self.inner
    }
}

impl<'a> SegKey<'a> {
    pub const fn new(prefix: &'a str) -> Self {
        SegKey {
            inner: prefix.as_bytes(),
        }
    }

    pub fn join(&self, value: impl AsRef<str>) -> SegKeyBuf {
        let value_bytes = value.as_ref().as_bytes();

        let total = self.inner.len() + 1 + value_bytes.len(); // Plus 1 for the separator

        let mut buf = SegKeyBuf::with_capacity(total);

        buf.extend_from_slice(self.inner);
        buf.push(SEGMENT_SEPARATOR);
        buf.extend_from_slice(value_bytes);
        buf
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.inner
    }

    pub fn as_str(&self) -> Result<&str> {
        Ok(std::str::from_utf8(self.inner)?)
    }

    pub fn to_segkey_buf(&self) -> SegKeyBuf {
        SegKeyBuf {
            inner: self.inner.to_vec().into(),
        }
    }
}

pub trait SegmentExt {
    fn after<S: AsRef<[u8]>>(&self, segment: S) -> Option<SegKeyBuf>;
}

impl<T> SegmentExt for T
where
    T: AsRef<[u8]>,
{
    fn after<S: AsRef<[u8]>>(&self, segment: S) -> Option<SegKeyBuf> {
        let bytes = self.as_ref();
        let seg = segment.as_ref();
        let sep = SEGMENT_SEPARATOR;
        let pat_len = seg.len();
        let max_start = bytes.len().saturating_sub(pat_len);

        for start in 0..=max_start {
            // Check if the segment is at the current position
            if &bytes[start..start + pat_len] == seg {
                // Make sure this is a segment boundary:
                // either the start of the buffer, or the previous byte is a separator
                let before_ok = start == 0 || bytes[start - 1] == sep;
                let after_pos = start + pat_len;
                // Make sure the next byte is a separator
                if before_ok && after_pos < bytes.len() && bytes[after_pos] == sep {
                    let tail = &bytes[after_pos + 1..];
                    let mut buf = SegKeyBuf::with_capacity(tail.len());
                    buf.extend_from_slice(tail);
                    return Some(buf);
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_segkey_new() {
        let key = SegKey::new("prefix");
        assert_eq!(key.as_bytes(), b"prefix");
    }

    #[test]
    fn test_segkey_join() {
        let key = SegKey::new("prefix");
        let joined = key.join("suffix");
        assert_eq!(joined.as_bytes(), b"prefix:suffix");
    }

    #[test]
    fn test_segkey_join_multiple() {
        let key = SegKey::new("prefix");
        let joined1 = key.join("middle");
        let joined2 = SegKey::new(std::str::from_utf8(joined1.as_bytes()).unwrap()).join("suffix");
        assert_eq!(joined2.as_bytes(), b"prefix:middle:suffix");
    }

    #[test]
    fn test_segkey_empty() {
        let key = SegKey::new("");
        let joined = key.join("value");
        assert_eq!(joined.as_bytes(), b":value");
    }

    #[test]
    fn test_segkey_with_capacity() {
        let buf = SegKeyBuf::with_capacity(100);
        assert!(buf.capacity() >= 100);
    }
}
