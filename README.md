A repository containing examples from [Steve Donovan's Rust guidebook](https://stevedonovan.github.io/rust-gentle-intro/).

This repository contains scripts to help compile and execute the Rust source files
while organizing binaries into convenient subdirectories to keep your workspace clean.

To run examples use one of the following methods:

- Invoke [rustc](https://www.rust-lang.org/tools/install) directly. I suggest you use the `--out-dir` option to place your binaries into a subdirectory.

- With [Powershell 7](https://docs.microsoft.com/en-us/powershell/scripting/install/installing-powershell?view=powershell-7.1):
	- Run `Set-ExecutionPolicy RemotedSigned` to enable script execution.
	- Run the provided `rrun_pwsh.ps1` script like so: `.\rrun_pwsh.ps1 [source file] [arguments for the binary]`.
	- Example `.\rrun_pwsh.ps1 .\1\args0.rs hello world`.




To run examples either invoke `rustc` directly or [install Powershell 7](https://docs.microsoft.com/en-us/powershell/scripting/install/installing-powershell-core-on-windows) and invoke the provided `rrun_pwsh.ps1` script.

Run the script as follows:
`.\rrun_pwsh.ps1 [Rust source file] [arguments to the Rust program]`

For example, if you have a Rust executable named `main.rs` which
accepts an integer as a command line argument run:
`.\rrun_pwsh.ps1 .\main.rs 123`

IMPORTANT NOTE:
Arguments passed to Rust executables INCLUDE both '.\rrun_pwsh.ps1' 
and the Rust source file path as a String. Therefore, if you iterate
through `env::args()` or use `env::args().nth()` you will need to use
the `.skip(2)` iterator method to skip these first two arguments.
