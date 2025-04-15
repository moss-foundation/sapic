pub const FILENAME_SPECIAL_CHARS: [&str; 10] = [
    ".",  // dot
    "/",  // path separator
    "\\", // backslash
    ":",  // colon
    "*",  // wildcard
    "?",  // question mark
    "\"", // quotes
    "<",  // angle brackets
    ">",  // angle brackets
    "|",  // pipe
];

// We don't allow a slash/backslash in a foldername
// They will always represent a path separator, not a character to be encoded
pub const FOLDERNAME_SPECIAL_CHARS: [&str; 8] = [
    ".",  // dot
    ":",  // colon
    "*",  // wildcard
    "?",  // question mark
    "\"", // quotes
    "<",  // angle brackets
    ">",  // angle brackets
    "|",  // pipe
];