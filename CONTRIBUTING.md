## Note for development on Windows

`libgit2` has a known bug on Windows which uses a faulty libssh2 backend, which won't be able to recognize new key
formats (https://github.com/rust-lang/git2-rs/issues/1139)
To make it work, we will need to compile a compatible version of openssl during build.
This requires first installing Strawberry Perl (https://strawberryperl.com/), and putting the bin folder in `PATH` env
Like: `C:\Strawberry\perl\bin` and `C:\Strawberry\c\bin`
It might take 10-15 minutes when compiling openssl, but normally it doesn't need to be rebuilt.