using Test

ENV["RUST_LOG"]=""
ENV["JULIA_SLINT_REBUILD"]=0
using Slint

s = "export component MyWin inherits Window {
        Text {
            text: \"Hello, World\";
        }
    }
    "
Slint.compile_from_string(s,"MyWin")



