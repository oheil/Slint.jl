# Slint

## Relevant online docs and tools

https://github.com/slint-ui/slint

https://slint.dev/releases/1.7.0/docs/slint/

https://docs.rs/

https://slint.dev/releases/1.7.0/docs/rust/slint/

https://slint.dev/releases/1.7.0/editor/

https://jakegoulding.com/rust-ffi-omnibus/objects/

https://discourse.julialang.org/t/how-to-create-a-cstring-from-a-string/98566

https://github.com/felipenoris/JuliaPackageWithRustDep.jl

## General Rust workflows
```shell
rustup default stable
rustup update
```

```shell
cd c:\Users\oheil\.julia\dev\Slint\deps\SlintWrapper
cargo update
cargo add slint-interpreter

cd c:\Users\oheil\.julia\dev\Slint\rust
cargo run --release --bin my_cells
cargo run --release --bin my_cells_listview
cargo run --release --bin circledraw
cargo run --release --bin circledraw_2dim_array
```

```shell
??? git clone -c core.symlinks=true https://github.com/slint-ui/slint
```

```shell
set RUST_LOG=debug
set RUST_LOG=
```

```shell
cd examples\7guis
cargo run --release --bin cells
```

## Build
How to build the projekt:
```julia
using Pkg; Pkg.build("Slint"; verbose = true);
include("contrib\\generator.jl")
```

```julia
using Slint
...
    # slintwrapper.dll is locked because loaded by `Libdl.dlopen_e`
...
Slint.close()   # release slintwrapper.dll

#a new build can be done now:
using Pkg; Pkg.build("Slint"; verbose = true);
include("contrib\\generator.jl")

Slint.__init__()  # load slintwrapper.dll again
...
    # test slint code
...
```

## Julia Examples

```julia
using Slint
s = "export component MyWin inherits Window {
        Text {
            text: \"Hello, World\";
        }
    }
    "
Slint.compile_from_string(s)
Slint.run()
```

```julia
using Slint
file1 = "examples\\7guis\\booker.slint"
file2 = "helloworld.slint"
file3 = "SingleButton.slint"

Slint.compile_from_file(file1)

#setting callbacks needs to be before the next call to CompileFromFile
Slint.compile_from_file(file2)
#after last command no callback can be set for file1 anymore!
function print_callback()
    println("Button clicked, Julia responded")
end
c_print_callback = @cfunction print_callback Cvoid ()
Slint.r_set_callback("button-clicked",c_print_callback)

Slint.compile_from_file(file3)

Slint.run()
```

```
# include("examples\\7guis\\booker.jl")
using Slint
file1 = "examples\\7guis\\booker.slint"
Slint.compile_from_file(file1)
function on_validate_date(date)
    println(date)
    return Cint(1)
end
Slint.SetCallback_specific("validate-date", on_validate_date, :Cint, :(Slint.SharedString,) )

Slint.Run()
```

```
using Slint
file1 = "examples\\7guis\\booker.slint"
file2 = "SingleButton.slint"

#setting callbacks needs to be before the next call to CompileFromFile
Slint.compile_from_file(file1)
#after last command no callback can be set for file1 anymore!
function on_validate_date(date)
    println("Validate called")
    println(date)
    return Cint(1)
end
Slint.set_callback("validate-date", on_validate_date, :Cint, :(Slint.SharedString,) )

#setting callbacks needs to be before the next call to CompileFromFile
Slint.compile_from_file(file2)
#after last command no callback can be set for file2 anymore!
function print_callback(text)
    println("Button clicked, Julia responded")
    println(text)
    return Cint(1)
end
Slint.set_callback("button-clicked", print_callback, :Cint, :(Slint.SharedString,) )

Slint.run()
```

# Old snippets

```
cargo run --release --target-dir "../../deps/" --bin my_cells
```



