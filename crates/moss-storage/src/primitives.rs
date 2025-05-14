mod namespace {
    use std::ops::{Deref, DerefMut};

    use smallvec::SmallVec;

    const NAMESPACE_SEPARATOR: u8 = b':';
    const NAMESPACE_BUF_CAPACITY: usize = 128;

    pub struct NamespaceBuf {
        inner: SmallVec<[u8; NAMESPACE_BUF_CAPACITY]>,
    }

    impl Deref for NamespaceBuf {
        type Target = SmallVec<[u8; NAMESPACE_BUF_CAPACITY]>;

        fn deref(&self) -> &Self::Target {
            &self.inner
        }
    }

    impl DerefMut for NamespaceBuf {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.inner
        }
    }

    impl NamespaceBuf {
        pub fn with_capacity(capacity: usize) -> Self {
            Self {
                inner: SmallVec::with_capacity(capacity),
            }
        }
    }

    pub struct Namespace {
        prefix: &'static [u8],
    }

    impl Namespace {
        pub const fn new(prefix: &'static str) -> Self {
            Namespace {
                prefix: prefix.as_bytes(),
            }
        }

        pub fn join(&self, value: &str) -> NamespaceBuf {
            let value_bytes = value.as_bytes();

            let total = self.prefix.len() + 1 + value_bytes.len(); // Plus 1 for the separator

            let mut buf = NamespaceBuf::with_capacity(total);

            buf.extend_from_slice(self.prefix);
            buf.push(NAMESPACE_SEPARATOR);
            buf.extend_from_slice(value_bytes);
            buf
        }
    }
}
pub use namespace::*;
