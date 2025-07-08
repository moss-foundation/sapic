use nanoid::nanoid;
use std::{fmt::Display, hash::Hash};

const ID_LENGTH: usize = 10;

pub fn new_nanoid_string() -> String {
    nanoid!(ID_LENGTH)
}
