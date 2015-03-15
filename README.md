`rustc <file>.rs` - compile rust code to bin file `<file>`

`rustc --crate-type=lib <file>.rs` - compile rust to lib file like `liberty-e6eaab2e-0.0.rlib`

  - `extern crate <file>` - load liberty file, and could be use like `file::function()`

`rustc -L . <file>.rs` - `-L .` added current directory to the library search path
