A repository containing examples from [Steve Donovan's Rust guidebook](https://stevedonovan.github.io/rust-gentle-intro/).

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
