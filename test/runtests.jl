using Test

ENV["RUST_LOG"]=""   # Disable logging for tests, you can set it to "debug" for more verbose output
ENV["JULIA_SLINT_REBUILD"]=0 # Disable rebuild of SlintWrapper, set to "1" to force rebuild

using Slint

s = raw"""
    export component MyWin inherits Window {
        Text {
            text: "Hello, World";
        }
    }
    """
Slint.compile_from_string(s,"MyWin")
rv = Slint.get_error_state()
if rv.magic != Slint.rMagic
    error("Slint.get_error_state() returned an unexpected magic number: $(rv.magic)")
end
if rv.rtype != C_NULL && unsafe_string(rv.rtype) != Slint.rtypes[Int(Slint.rErrorState)]
    error("Slint.get_error_state() returned an unexpected rtype: $(unsafe_string(rv.rtype))")
end
if rv.int_value == 1
    error("Slint.get_error_state() returned an error state: $(unsafe_string(rv.string_value))")
end





