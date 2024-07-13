# Slint.jl
Slint GUI for Julia


# download/install
using Pkg
Pkg.add(url="https://github.com/oheil/Slint.jl.git")
dev Slint

# build
cd(".julia/dev/Slint")
activate .
using Pkg; Pkg.build("Slint"; verbose = true);
include("contrib\\generator.jl")

# run examples
include("examples\\7guis\\booker.jl")
include("examples\\7guis\\cells.jl")

# REPL examples

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

