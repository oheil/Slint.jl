# Slint

## Relevant online docs and tools

[https://github.com/slint-ui/slint]

[https://docs.slint.dev/latest/docs/slint/]

[https://docs.rs/]

[https://docs.slint.dev/latest/docs/rust/slint/]

[https://docs.slint.dev/latest/editor/]

[https://jakegoulding.com/rust-ffi-omnibus/objects/]

[https://discourse.julialang.org/t/how-to-create-a-cstring-from-a-string/98566]

[https://github.com/felipenoris/JuliaPackageWithRustDep.jl]

## Julia Registrator workflow

- Edit Project.toml with next version number, e.g. version = "0.1.2" => version = "0.1.3"
- Commit and Sync
- On GitHub comment on latest commit with e.g.:

```text
@JuliaRegistrator register

# Release notes v0.1.5:

## General notes

- important REPL example working again

## Bug fixes

- minor bugs in examples
- ...
```

- wait and repeat until Registrator and TagBot does the work

## General Rust workflows

```shell
rustup default stable
rustup update
```

```shell
cd c:\Users\oheil\.julia\dev\Slint
cd deps\SlintWrapper
cargo update
cargo add slint-interpreter
cd ..\..

cd c:\Users\oheil\.julia\dev\Slint\rust
cargo update

cargo run --release --bin my_cells
cargo run --release --bin my_cells_listview
cargo run --release --bin circledraw
cargo run --release --bin circledraw_2dim_array
cargo run --release --bin crud
cargo run --release --bin cells

cargo add plotters
cargo run --release --bin plotter

# can't get this to run because of env SLINT_INCLUDE_GENERATED not properly set
cargo run --release --bin gallery
```

```shell
??? git clone -c core.symlinks=true https://github.com/slint-ui/slint
```

```shell
set RUST_LOG=debug
set RUST_LOG=

set JULIA_SLINT_REBUILD=1
```

```powershell
[Environment]::SetEnvironmentVariable("RUST_LOG", "debug")
[Environment]::SetEnvironmentVariable("RUST_LOG", "")
```

## TEST: Upgrading from CEnum v0.5.0 to v1.0.0

```julia
(@v1.13) pkg> activate .
(Slint) pkg> activate contrib\\.
(contrib) pkg> add Clang#master
(contrib) pkg> up
(contrib) pkg> st
Status `C:\Z-Temp\Temp\Slint.jl\contrib\Project.toml`
  [fa961155] CEnum v1.0.0
  [40e3b903] Clang v0.19.3 `https://github.com/JuliaInterop/Clang.jl.git#master`
  [8f399da3] Libdl v1.11.0
(contrib) pkg> activate .
# remove 
#  [compat]
#  CEnum = "0.5.0"
# from Project.toml
(Slint) pkg> up
(Slint) pkg> st
Project Slint v0.1.13
Status `C:\Z-Temp\Temp\Slint.jl\Project.toml`
  [fa961155] CEnum v1.0.0
  [8f399da3] Libdl v1.11.0
```

Result: no problems

## Build

How to build the projekt:

- delete deps\release to rebuild all

```julia
ENV["JULIA_SLINT_REBUILD"]=1
using Pkg; Pkg.activate("."); Pkg.build("Slint"; verbose = true);
include("contrib/generator.jl")
```

```julia
using Pkg;
Pkg.test("Slint")
Pkg.test("Slint"; test_args=["-v"])   # verbose tests
Pkg.test("Slint"; test_args=["-vv"])  # more verbose tests
```

```julia
using Slint
...
    # slintwrapper.dll is locked because loaded by `Libdl.dlopen_e`
...
Slint.close()   # release slintwrapper.dll

#a new build can be done now:
ENV["JULIA_SLINT_REBUILD"]=1
using Pkg; Pkg.build("Slint"; verbose = true);
include("contrib/generator.jl")

Slint.__init__()  # load slintwrapper.dll again
...
    # test slint code
...
```

## Julia Examples

```julia
include("examples/gallery/main.jl")
include("examples/plotter/main.jl")

include("examples/7guis/timer.jl")
include("examples/7guis/tempconv.jl")
include("examples/7guis/crud.jl")
include("examples/7guis/counter.jl")
include("examples/7guis/circledraw.jl")
include("examples/7guis/cells.jl")
include("examples/7guis/booker.jl")
```

```julia
using Slint
s = "export component MyWin inherits Window {
        Text {
            text: \"Hello, World\";
        }
    }
    "
Slint.compile_from_string(s,"MyWin")
Slint.run()
```

```julia
using Slint
file1 = "examples/7guis/booker.slint"
file2 = "helloworld.slint"
file3 = "SingleButton.slint"

Slint.compile_from_file(file1,"Booker")

#setting callbacks needs to be before the next call to CompileFromFile
Slint.compile_from_file(file2,"Demo")
#after last command no callback can be set for file1 anymore!
function print_callback()
    println("Button clicked, Julia responded")
    return true
end
Slint.set_callback("button-clicked",print_callback)

Slint.compile_from_file(file3,"SingleButton")

Slint.run()
```

```julia
# include("examples/7guis/booker.jl")
using Slint
file1 = "examples/7guis/booker.slint"
Slint.compile_from_file(file1,"Booker")
function on_validate_date(date)
    println(date)
    return true
end
Slint.set_callback("validate-date", on_validate_date)

Slint.run()
```

## Old snippets

```julia
using Slint
file1 = "examples/7guis/booker.slint"
file2 = "SingleButton.slint"

#setting callbacks needs to be before the next call to CompileFromFile
Slint.compile_from_file(file1,"Booker")
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

```cmd
cargo run --release --target-dir "../../deps/" --bin my_cells
```
