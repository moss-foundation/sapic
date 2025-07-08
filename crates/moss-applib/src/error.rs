use derive_more::Deref;
use std::{
    any::TypeId,
    fmt::{self, Display},
    marker::PhantomData,
};

// #[derive(Debug, Clone, Copy, PartialEq, Deref)]
// pub struct ErrorCode(&'static str);

// impl Default for ErrorCode {
//     fn default() -> Self {
//         UNKNOWN
//     }
// }

// impl Display for ErrorCode {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{}", self.0)
//     }
// }

pub trait Marker: 'static {
    const CODE: &'static str;
}

impl Marker for () {
    const CODE: &'static str = "_";
}

pub struct ErrorInvalidInput;
pub struct ErrorNotFound;

impl Marker for ErrorInvalidInput {
    const CODE: &'static str = "errors.invalid_input";
}
impl Marker for ErrorNotFound {
    const CODE: &'static str = "errors.not_found";
}

pub trait ResultExt<T> {
    fn join<E: Marker>(self, message: impl Into<String>) -> Result<T, Error>;
}

impl<T> ResultExt<T> for Result<T, Error> {
    fn join<E: Marker>(self, message: impl Into<String>) -> Result<T, Error> {
        self.map_err(|e| e.join::<E>(message.into()))
    }
}

impl<T> ResultExt<T> for anyhow::Result<T> {
    fn join<E: Marker>(self, message: impl Into<String>) -> Result<T, Error> {
        self.map_err(|e| Error::new::<()>(e.to_string()).join::<E>(message))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Error {
    code: Option<&'static str>,
    message: String,
    type_id: TypeId,
    source: Option<Box<Error>>,
}

impl Error {
    pub fn new<E: Marker>(message: impl Into<String>) -> Self {
        let type_id = TypeId::of::<E>();
        Self {
            code: if type_id != TypeId::of::<()>() {
                Some(E::CODE)
            } else {
                None
            },
            message: message.into(),
            type_id,
            source: None,
        }
    }

    pub fn join<E: Marker>(self, message: impl Into<String>) -> Self {
        let type_id = TypeId::of::<E>();
        Error {
            code: if type_id != TypeId::of::<()>() {
                Some(E::CODE)
            } else {
                None
            },
            message: message.into(),
            type_id,
            source: Some(Box::new(self)),
        }
    }

    pub fn is<E: Marker>(&self) -> bool {
        if self.type_id == TypeId::of::<E>() {
            return true;
        }

        let mut current = &self.source;
        while let Some(source) = current {
            if source.type_id == TypeId::of::<E>() {
                return true;
            }
            current = &source.source;
        }

        false
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.code {
            Some(code) => write!(f, "{}: {}", code, self.message)?,
            None => write!(f, "{}", self.message)?,
        }

        let mut current = &self.source;
        while let Some(source) = current {
            match &source.code {
                Some(code) => write!(f, ": {}: {}", code, source.message)?,
                None => write!(f, ": {}", source.message)?,
            }
            current = &source.source;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use anyhow::bail;

    use super::*;

    fn test_error_anyhow() -> anyhow::Result<()> {
        bail!("Something went wrong");
    }

    fn test_error_2() -> Result<(), Error> {
        test_error_anyhow().join::<ErrorInvalidInput>("Something went wrong two")
    }

    #[test]
    fn test_error_chain() {
        let err = test_error_2()
            .join::<ErrorNotFound>("something not found")
            .err()
            .unwrap();

        println!("{}", err.to_string());

        assert_eq!(
            err.to_string(),
            "errors.not_found: something not found: errors.invalid_input: Something went wrong two: Something went wrong"
        );

        assert!(err.is::<ErrorNotFound>());
    }

    // #[test]
    // fn test_error_without_code() {
    //     let err = Error::new("Something went wrong without code")
    //         .with_context("context without code")
    //         .with_context("error from function");

    //     println!("{}", err);

    //     assert_eq!(
    //         err.to_string(),
    //         "error from function: context without code: Something went wrong without code"
    //     );
    // }
}
