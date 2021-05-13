# A Gentle Introduction to Rust
Examples from [Steve Donovan's Rust guidebook](https://stevedonovan.github.io/rust-gentle-intro/).

This repository contains scripts to help compile and execute the Rust source files
while organizing binaries into convenient subdirectories to keep your workspace clean.

To run examples please [install Rust](https://www.rust-lang.org/tools/install), ensure `rustc`
is added to `PATH` and then use one of the following methods:

## Rustc
Invoke [rustc](https://www.rust-lang.org/tools/install) directly. I suggest you use the `--out-dir` option to place your binaries into a subdirectory.

## Powershell 7
[Install Powershell 7](https://docs.microsoft.com/en-us/powershell/scripting/install/installing-powershell?view=powershell-7.1).
- Run `Set-ExecutionPolicy RemotedSigned` to enable script execution.
- Run the provided `rrun_pwsh.ps1` script like so: `.\rrun_pwsh.ps1 [source file] [arguments]`.
- Example `.\rrun_pwsh.ps1 .\1\args0.rs hello world`.

## BASH
- Run `chmod u+x ./rrun_bash` to make the script "executable".
- Run `./rrun_bash [source file] [arguments]`.
- Example `./rrun_bash ./1/args0.rs hello world`.

## BASH on WSL
This method works the same as the above BASH method, however the provided `rrun_wsl`
scripts contains a few necessary tweaks. If you receive errors similar to "unknown command: '\r'
then run `dos2unix ./rrun_wsl` to convert the newline characters. See the
[dos2unix manual page](https://linux.die.net/man/1/dos2unix) for more information.

## Suggestions

I highly suggest you remove any `rrun_` scripts you do not intend to use to better
leverage autocomplete. It may also help to rename the script to just `rrun.ps1` or
`rrun` for brevity.

IMPORTANT NOTE:
Arguments passed to Rust executables include the source file's path
and, in Powershell's case the script's path as well.

Therefore, if you iterate
through `env::args()` or use `env::args().nth()` you will need to use
the `.skip(2)` iterator method to skip these first two arguments.
