use nanoid::nanoid;

const ID_LENGTH: usize = 10;

pub fn new_nanoid() -> String {
    nanoid!(ID_LENGTH)
}
