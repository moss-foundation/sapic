use nanoid::nanoid;

const ID_LENGTH: usize = 10;

pub fn new_nanoid_string() -> String {
    nanoid!(ID_LENGTH)
}
