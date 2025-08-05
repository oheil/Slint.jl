using Test

ENV["RUST_LOG"]=""   # Disable logging for tests, you can set it to "debug" for more verbose output
ENV["JULIA_SLINT_REBUILD"]=0 # Disable rebuild of SlintWrapper, set to "1" to force rebuild

verbose = false
if "-v" in ARGS
    verbose = true
end
if "-vv" in ARGS
    verbose = true
    ENV["RUST_LOG"]="debug"
end

using Slint

@testset "Slint compilation from a string" begin
    #
    # Test mini Slint compilation from a string
    #

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
        @warn("Slint.get_error_state() returned an unexpected magic number: $(rv.magic)")
    end
    @test rv.magic == Slint.rMagic

    if rv.rtype != C_NULL && unsafe_string(rv.rtype) != Slint.rtypes[Int(Slint.rErrorState)]
        @warn("Slint.get_error_state() returned an unexpected rtype: $(unsafe_string(rv.rtype))")
    end
    @test rv.rtype != C_NULL && unsafe_string(rv.rtype) == Slint.rtypes[Int(Slint.rErrorState)]

    if rv.int_value == 1
        @warn("Slint.get_error_state() returned an error state: $(unsafe_string(rv.string_value))")
    end
    @test rv.int_value == 0
end;

@testset "Slint compilation from a file   " begin
    #
    # Test maxi Slint compilation from a file
    #

    slintFile = "../examples/gallery/gallery.slint"
    startComponent = "App"
    Slint.compile_from_file(slintFile,startComponent)
    rv = Slint.get_error_state()

    if rv.int_value == 1
        @warn("Slint.get_error_state() returned an error state: $(unsafe_string(rv.string_value))")
    end
    @test rv.int_value == 0
end;

@testset "Register a callback            " begin
    #
    # Test register a callback function
    #

    slintFile = "../examples/7guis/booker.slint"
    startComponent = "Booker"
    Slint.compile_from_file(slintFile,startComponent)
    rv = Slint.get_error_state()

    if rv.int_value == 1
        @warn("Slint.get_error_state() returned an error state: $(unsafe_string(rv.string_value))")
    end
    @test rv.int_value == 0

    function on_validate_date(params...)
        return true
    end
    Slint.set_callback("validate-date", on_validate_date)
    rv = Slint.get_error_state()

    if rv.int_value == 1
        @warn("Slint.get_error_state() returned an error state: $(unsafe_string(rv.string_value))")
    end
    @test rv.int_value == 0

    Slint.set_callback("wrong-callback-by-purpose", on_validate_date)
    rv = Slint.get_error_state()

    if rv.int_value == 1
        if verbose
            @info("This error is provoked by purpose, the callback 'wrong-callback-by-purpose' is not defined in the Slint file.")
            @info("  Slint.get_error_state() returned an error state: $(unsafe_string(rv.string_value))")
        end
    end
    @test rv.int_value == 1
end;

@testset "Clear the error state           " begin
    #
    # Clear error state
    #
    rv = Slint.get_error_state()
    if rv.int_value == 0
        @warn("Slint.get_error_state() returned NOT an error state: $(unsafe_string(rv.string_value))")
    end
    @test rv.int_value == 1

    Slint.clear_error_state()
    rv = Slint.get_error_state()
    if rv.int_value == 1
        @warn("Slint.get_error_state() returned an error state: $(unsafe_string(rv.string_value))")
    end
    @test rv.int_value == 0
end;

@testset "Set/get values of properties   " begin
    #
    # Test setting and getting values of properties
    #
    s = raw"""
        import { LineEdit } from "std-widgets.slint";
        export component MyWin inherits Window {
            in-out property <string> usertext: "some text";
            Text {
                LineEdit {
                    text <=> root.usertext;
                }
            }
        }
        """
    Slint.compile_from_string(s,"MyWin")
    usertext = Slint.get_value("usertext")
    @test usertext == "some text"

    usertext = Slint.get_value("unknown-property")
    rv = Slint.get_error_state()
    if rv.int_value == 1
        if verbose
            @info("This error is provoked by purpose, the property 'unknown-property' is not defined in the Slint string.")
            @info("Slint.get_error_state() returned an error state: $(unsafe_string(rv.string_value))")
        end
    end
    @test rv.int_value == 1

    Slint.set_value("usertext","new text")
    usertext = Slint.get_value("usertext")
    @test usertext == "new text"

    Slint.set_value("unknown-property","new text")
    rv = Slint.get_error_state()
    if rv.int_value == 1
        if verbose
            @info("This error is provoked by purpose, the property 'unknown-property' is not defined in the Slint string.")
            @info("Slint.get_error_state() returned an error state: $(unsafe_string(rv.string_value))")
        end
    end
    @test rv.int_value == 1
end;

@testset "Set/get cell values             " begin
    #
    # Test setting and getting cell values
    #


end;
