# Slint.jl

## This package is in a VERY EARLY development state! You need rust installed if you want to try it out

Slint GUI for Julia

## prerequisites

- Rust
- Visual Studio Community 2017 (or later) with C++ Build Tools ( [see below](https://github.com/oheil/Slint.jl?tab=readme-ov-file#install-build-prerequsites-in-visual-studio-installer) )

## download/install

```julia
using Pkg
Pkg.add(url="https://github.com/oheil/Slint.jl.git")  # build errors (see below) should be resolved now
```

### Remarks to build errors after `Pkg.add(url="https://github.com/oheil/Slint.jl.git")`

On Windows, packages are added to folders like `.julia\packages\Slint\uZ1Dp\`. All folders have full access rights for the current user, but files only have restricted access rights, typically read only. This prevents the build process to succeed because some build artefacts need to be overwritten during build which will fail because of insufficient access rights.
In this case the problematic file is
```
.julia\packages\Slint\uZ1Dp\deps\SlintWrapper\include\slintwrapper.h
```
and a solution will be found at some time.

## working examples

```julia
using Slint
cd(joinpath(dirname(pathof(Slint)), ".."))

include("examples\\7guis\\booker.jl")
include("examples\\7guis\\cells.jl")
include("examples\\7guis\\circledraw.jl")
include("examples\\7guis\\counter.jl")
```

## Development and Build

```julia
using Pkg
Pkg.develop("Slint")
```

```julia
using Pkg
cd(".julia/dev/Slint")
Pkg.activate(".")
Pkg.build("Slint"; verbose = true);
include("contrib\\generator.jl")
```

## current example work in progress

```julia
include("examples\\7guis\\crud.jl")
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

```julia
using Slint
file1 = "examples\\7guis\\booker.slint"
file2 = "helloworld.slint"
file3 = "SingleButton.slint"

Slint.compile_from_file(file1,"Booker")

#setting callbacks needs to be before the next call to CompileFromFile
Slint.compile_from_file(file2,"Demo")
#after last command no callback can be set for file1 anymore!
function print_callback()
    println("Button clicked, Julia responded")
end
Slint.set_callback("button-clicked",print_callback)

Slint.compile_from_file(file3,"SingleButton")

Slint.run()
```

## Install Build prerequsites in Visual Studio Installer

![wi1](https://github.com/user-attachments/assets/fed0a9ed-8c6d-40b5-bd3c-4ef5b8d69351)
![wi2](https://github.com/user-attachments/assets/ba48c61c-145a-4310-a96e-b7df646852cd)
