# Slint.jl

Slint GUI for Julia

Providing a library for Julia to use Slint ([GitHub](https://github.com/slint-ui/slint), [Webpage](https://slint.dev/)), a GUI library for rust.

## For Windows and Linux no build is needed anymore

Make sure you have at least v0.1.4 installed!

## Download and Install

```julia
using Pkg
Pkg.add("Slint")
```

## Examples

See [below for WSL (Windows Subsystem for Linux) prerequisites](https://github.com/oheil/Slint.jl?tab=readme-ov-file#for-wsl-debian-trixie-to-show-the-slint-gui) for the Slint GUIs to show up in WSL.

A small and non-trivial Julia program using Slint GUI looks as clear and comprehensible as this example:

```julia
using Slint, Dates

cd(joinpath(dirname(pathof(Slint)), ".."))

slintFile = "examples/7guis/booker.slint" # the Slint GUI declaration is here inside
startComponent = "Booker"

# Main/Start component is "Booker"
Slint.compile_from_file(slintFile,startComponent)

# implementation of callback:
#       pure callback validate-date(string) -> bool;
function on_validate_date(params...)
    if isnothing(match(r"^\d\d\.\d\d.\d\d\d\d$",params[1]))
        return false
    end
    return true
end
# register callback for:
#       pure callback validate-date(string) -> bool;
Slint.set_callback("validate-date", on_validate_date)

# implementation of callback:
#       pure callback compare-date(string, string) -> bool;
function on_compare_date(params...)
    d1 = Dates.tryparse(Date,params[1],dateformat"d.m.Y")
    d2 = Dates.tryparse(Date,params[2],dateformat"d.m.Y")
    if isnothing(d1) || isnothing(d2) || d1 > d2
        return false
    end
    return true
end
# register callback for:
#       pure callback compare-date(string, string) -> bool;
Slint.set_callback("compare-date", on_compare_date)

# run the application
Slint.run()

# unload library
Slint.close()
```

This example shows a complete (at the time) overview of all GUI components:

```julia
using Slint
cd(joinpath(dirname(pathof(Slint)), ".."))
include("examples/gallery/main.jl")
```

All examples available:

```julia
using Slint
cd(joinpath(dirname(pathof(Slint)), ".."))

include("examples/7guis/booker.jl")
include("examples/7guis/cells.jl")
include("examples/7guis/circledraw.jl")
include("examples/7guis/counter.jl")
include("examples/7guis/crud.jl")
include("examples/7guis/tempconv.jl")
include("examples/7guis/timer.jl")

include("examples/plotter/main.jl")

# showcase all Slint widgets
include("examples/gallery/main.jl")
```

## REPL examples

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

This will work with v0.1.5 again:

```julia
using Slint

cd(joinpath(dirname(pathof(Slint)), ".."))

file1 = "examples/7guis/booker.slint"
file2 = "examples/helloworld.slint"
file3 = "examples/SingleButton.slint"

Slint.compile_from_file(file1,"Booker")

#setting callbacks needs to be before the next call to CompileFromFile
Slint.compile_from_file(file2,"Demo")
#after last command no callback can be set for file1 anymore!
function print_callback()
    println("Button clicked, Julia responded")
    # return true, not nothing
    return true
end
Slint.set_callback("button-clicked",print_callback)

Slint.compile_from_file(file3,"SingleButton")

Slint.run()
```

## Current known issues

- Linux: when closing the Slint window and running again => segmentation fault (problem with Libdl.dlclose)

    The library can't be unloaded.

    See: [https://github.com/JuliaLang/julia/issues/44722](https://github.com/JuliaLang/julia/issues/44722)

    First clang experiments did not resolve:

    /home/oheil/.cargo/config.toml:

    ```TOML
    rustflags = [
    "-C", "linker=clang",
    ]
    ```

    ```bash
    export JULIA_SLINT_REBUILD=1
    export CC=/usr/bin/clang
    export CXX=/usr/bin/clang++
    ```

    Building everything didn't change a thing.

- RESOLVED Windows: build errors after `Pkg.add(url="https://github.com/oheil/Slint.jl.git")`

    On Windows, packages are added to folders like `.julia\packages\Slint\uZ1Dp\`. All folders have full access rights for the current user, but files only have restricted access rights, typically read only. This prevents the build process to succeed because some build artefacts need to be overwritten during build which will fail because of insufficient access rights.
    In this case the problematic file is

    ```julia
    .julia\packages\Slint\uZ1Dp\deps\SlintWrapper\include\slintwrapper.h
    ```

    and a solution will be found at some time.

## Build prerequisites

For Linux it's recommended to build the wrapper library on your system.

In general you need:

- Rust
- Windows: Visual Studio Community 2017 (or later) with C++ Build Tools ( [see below](https://github.com/oheil/Slint.jl?tab=readme-ov-file#install-build-prerequisites-in-visual-studio-installer) )
- Linux: [see below](https://github.com/oheil/Slint.jl?tab=readme-ov-file#install-build-prerequisites-for-linux-tested-with-cachyos)

## Development and Build

To rebuild libraries set the environment variable `JULIA_SLINT_REBUILD`:

```bash
export JULIA_SLINT_REBUILD=1  #bash
```

```shell
set JULIA_SLINT_REBUILD=1   #csh + cmd
```

```julia
julia> ENV["JULIA_SLINT_REBUILD"]=1  #julia REPL
```

```julia
using Pkg
Pkg.develop("Slint")
```

```julia
using Pkg
cd(".julia/dev/Slint")
Pkg.activate(".")
Pkg.build("Slint"; verbose = true);

include("contrib/generator.jl") # only needed if you have changed the API (i.e. changes in deps\SlintWrapper\src\lib.rs)
```

More verbose debug output of the rust library:

```shell
set RUST_LOG=debug
```

## Install Build prerequisites for Linux (tested with CachyOS)

as root:

```fish
pacman -S rustup
```

as developer/user:

```fish
rustup default stable
```

## Install Build prerequisites for Linux (tested with Ubuntu 24.04 LTS (Noble Numbat), Debian Trixie in WSL)

as root:

```bash
apt install rustup
apt install build-essential cmake
apt install libfontconfig1-dev
```

as developer/user:

```bash
#installing juliaup:
curl -fsSL https://install.julialang.org | sh
rustup default stable
```

## Cloning and building the project

```bash
git clone https://github.com/oheil/Slint.jl.git
cd Slint.jl

export JULIA_SLINT_REBUILD=1  #bash or set it in Julia REPL

julia
```

in Julia:

```julia
julia> ENV["JULIA_SLINT_REBUILD"]=1  # if not set in the shell
julia> using Pkg; Pkg.activate("."); Pkg.build("Slint"; verbose = true)
julia> include("examples/7guis/booker.jl")
```

### For WSL Debian Trixie to show the Slint GUI

as root

```bash
apt install x11-apps # shortcut to get some needed x11 libs as dependency
apt install libxkbcommon-x11-dev
apt install freeglut3-dev
apt install xdg-desktop-portal-gtk  # only if you want to get rid of error: Error watching for xdg color schemes: org.freedesktop.DBus.Error.ServiceUnknown: The name org.freedesktop.portal.Desktop was not provided by any .service files
```

## Running Tests

```julia
using Pkg;
Pkg.test("Slint")
Pkg.test("Slint"; test_args=["-v"])   # verbose tests
Pkg.test("Slint"; test_args=["-vv"])  # more verbose tests
```

## Install Build prerequisites in Visual Studio Installer

![wi1](https://github.com/user-attachments/assets/fed0a9ed-8c6d-40b5-bd3c-4ef5b8d69351)
![wi2](https://github.com/user-attachments/assets/ba48c61c-145a-4310-a96e-b7df646852cd)
