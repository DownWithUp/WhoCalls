# WhoCalls
WhoCalls can query a directory of files, find the binaries, and search for a user specified Win API import. It works with both 32-bit (PE) and 64-bit (PE32+) file formats (.exe, .dll, .sys)
This is an improved version of my old C WhoCalls program. The old C version can be found [here](https://github.com/DownWithUp/WhoCalls_C).

## Example Use
WhoCalls.exe [Path To Query] [API Name] <br>
API Name: A valid Windows API function. <br>
Path To Query: A valid path to a directory containg the executable(s). <br>
E.g. `whocalls C:\Windows\System32 ExitProcess`

## Other Features
* No unsafe Rust 😊
* Argument parsing by [clap](https://github.com/clap-rs/clap)
* PE parsing by [goblin](https://github.com/m4b/goblin)
