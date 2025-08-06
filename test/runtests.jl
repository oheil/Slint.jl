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

    Slint.clear_error_state()

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

    Slint.clear_error_state()

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

    Slint.clear_error_state()

    Slint.set_value("unknown-property","new text")

    rv = Slint.get_error_state()
    if rv.int_value == 1
        if verbose
            @info("This error is provoked by purpose, the property 'unknown-property' is not defined in the Slint string.")
            @info("Slint.get_error_state() returned an error state: $(unsafe_string(rv.string_value))")
        end
    end
    @test rv.int_value == 1
    
    Slint.clear_error_state()
end;

@testset "Set/get cell values             " begin
    #
    # Test setting and getting cell values
    #
    slintFile = "../examples/7guis/cells.slint"
    startComponent = "MainWindow"

    Slint.compile_from_file(slintFile, startComponent)

    rows = 10
    columns = 5

    function on_changed_element(params...)
        return true
    end
    Slint.set_property_model("cells",rows,columns,on_changed_element)

    rv = Slint.get_error_state()
    @test rv.int_value == 0

    for row in 1:rows
        for col in 1:columns
            Slint.set_cell_value("cells", row, col, string(row * col))
            cell_value = Slint.get_cell_value("cells", row, col)
            @test cell_value == string(row * col)
        end
    end

    Slint.clear_error_state()

    Slint.set_cell_value("cells", rows+1, columns+1, "cell not existing")

    rv = Slint.get_error_state()
    if rv.int_value == 1
        if verbose
            @info("set_cell_value")
            @info("This error is provoked by purpose, the cell at row $(rows+1), column $(columns+1) does not exist.")
            @info("Slint.get_error_state() returned an error state: $(unsafe_string(rv.string_value))")
        end
    end
    @test rv.int_value == 1

    Slint.clear_error_state()

    Slint.get_cell_value("cells", rows+1, columns+1)

    rv = Slint.get_error_state()
    if rv.int_value == 1
        if verbose
            @info("get_cell_value")
            @info("This error is provoked by purpose, the cell at row $(rows+1), column $(columns+1) does not exist.")
            @info("Slint.get_error_state() returned an error state: $(unsafe_string(rv.string_value))")
        end
    end
    @test rv.int_value == 1

    Slint.clear_error_state()

    Slint.set_cell_value("Not existing", rows, columns, "cell not existing")

    rv = Slint.get_error_state()
    if rv.int_value == 1
        if verbose
            @info("set_cell_value")
            @info("This error is provoked by purpose, the property 'Not existing' does not exist.")
            @info("Slint.get_error_state() returned an error state: $(unsafe_string(rv.string_value))")
        end
    end
    @test rv.int_value == 1

    Slint.clear_error_state()

    Slint.get_cell_value("Not existing", rows, columns)

    rv = Slint.get_error_state()
    if rv.int_value == 1
        if verbose
            @info("get_cell_value")
            @info("This error is provoked by purpose, the property 'Not existing' does not exist.")
            @info("Slint.get_error_state() returned an error state: $(unsafe_string(rv.string_value))")
        end
    end
    @test rv.int_value == 1

    Slint.clear_error_state()
end;

@testset "StandardListViewItem bridge    " begin
    #
    # Test the StandardListViewItem bridge
    #
    s = raw"""
            import { StandardListView } from "std-widgets.slint";

            struct SlintValue  { value_s: string, value_i: int, value_f: float }

            export component MainWindow inherits Window {
                in property <[StandardListViewItem]> names-list;

                callback bridge2StandardListViewItem( string, string );

                in property <[[SlintValue]]> names-list-bridge;
                changed names-list-bridge => {
                    bridge2StandardListViewItem("names-list-bridge","names-list");
                }

                StandardListView {
                    model: root.names-list;
                }
            }
        """
    Slint.compile_from_string(s,"MainWindow")

    Slint.set_property_model("names-list-bridge",1,1)

    rv = Slint.get_error_state()
    @test rv.int_value == 0

    Slint.push_rows("names-list-bridge",["Emil, Hans", "Mustermann, Max", "Tisch, Roman"])

    rv = Slint.get_error_state()
    @test rv.int_value == 0

    Slint.clear_rows("names-list-bridge")

    rv = Slint.get_error_state()
    @test rv.int_value == 0

    Slint.clear_error_state()

    Slint.set_property_model("other-list-bridge",1,1)

    rv = Slint.get_error_state()
    if rv.int_value == 1
        if verbose
            @info("set_property_model")
            @info("This error is provoked by purpose, the property 'other-list-bridge' does not exist.")
            @info("Slint.get_error_state() returned an error state: $(unsafe_string(rv.string_value))")
        end
    end
    @test rv.int_value == 1

    Slint.clear_error_state()

    Slint.push_rows("other-list-bridge",["Emil, Hans", "Mustermann, Max", "Tisch, Roman"])

    rv = Slint.get_error_state()
    if rv.int_value == 1
        if verbose
            @info("push_rows")
            @info("This error is provoked by purpose, the property 'other-list-bridge' does not exist.")
            @info("Slint.get_error_state() returned an error state: $(unsafe_string(rv.string_value))")
        end
    end
    @test rv.int_value == 1

    Slint.clear_error_state()

    Slint.clear_rows("other-list-bridge")
    rv = Slint.get_error_state()
    if rv.int_value == 1
        if verbose
            @info("clear_rows")
            @info("This error is provoked by purpose, the property 'other-list-bridge' does not exist.")
            @info("Slint.get_error_state() returned an error state: $(unsafe_string(rv.string_value))")
        end
    end
    @test rv.int_value == 1

    Slint.clear_error_state()
end;
