## Note for development on Windows

### Git SSH Key Compatibility

`libgit2` has a known bug on Windows which uses a faulty libssh2 backend, which won't be able to recognize new key
formats (https://github.com/rust-lang/git2-rs/issues/1139)
To make it work, we will need to compile a compatible version of openssl during build.
This requires first installing Strawberry Perl (https://strawberryperl.com/), and putting the bin folder in `PATH` env
Like: `C:\Strawberry\perl\bin` and `C:\Strawberry\c\bin`
It might take 10-15 minutes when compiling openssl, but normally it doesn't need to be rebuilt.

### Unix Commands in PowerShell

When running Makefile commands, you may encounter errors like `'rm' is not recognized as an internal or external command`. This happens because Windows PowerShell doesn't natively support Unix commands.

**Solution:** Install GnuWin32 CoreUtils package to enable Unix commands in Windows:
1. Download from: https://gnuwin32.sourceforge.net/packages/coreutils.htm
2. Get "Complete package, except sources": https://sourceforge.net/projects/gnuwin32/files/coreutils/5.3.0/coreutils-5.3.0.exe
3. Install and add the bin directory to your PATH environment variable (e.g., `C:\Program Files (x86)\GnuWin32\bin`)