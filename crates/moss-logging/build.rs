use std::process::{Command, ExitCode};

fn main() -> ExitCode {
    // #[cfg(target_os = "windows")]
    // let python_cmd = "python";
    //
    // #[cfg(not(target_os = "windows"))]
    // let python_cmd = "python3";
    //
    // Command::new(python_cmd)
    //     .args(["../../misc/ts_imports_injector.py", "package.json"])
    //     .status()
    //     .map(|status| {
    //         if status.success() {
    //             ExitCode::SUCCESS
    //         } else {
    //             eprintln!("Command exited with non-zero status: {:?}", status);
    //             status
    //                 .code()
    //                 .map_or(ExitCode::FAILURE, |code| ExitCode::from(code as u8))
    //         }
    //     })
    //     .unwrap_or_else(|err| {
    //         eprintln!("Failed to execute command: {}", err);
    //         ExitCode::FAILURE
    //     });
    // Command::new(python_cmd)
    //     .args(["../../misc/ts_exports_injector.py"])
    //     .status()
    //     .map(|status| {
    //         if status.success() {
    //             ExitCode::SUCCESS
    //         } else {
    //             eprintln!("Command exited with non-zero status: {:?}", status);
    //             status
    //                 .code()
    //                 .map_or(ExitCode::FAILURE, |code| ExitCode::from(code as u8))
    //         }
    //     })
    //     .unwrap_or_else(|err| {
    //         eprintln!("Failed to execute command: {}", err);
    //         ExitCode::FAILURE
    //     })
    ExitCode::SUCCESS

}
