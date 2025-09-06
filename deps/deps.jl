import Libdl

const rustlibname = "slintwrapper"
const libname = Sys.iswindows() ? rustlibname : "lib" * rustlibname

function dylib_filenames()
    @static if Sys.isapple()
        "$libname.dylib"
    elseif Sys.islinux()
        "$libname.so"
    elseif Sys.iswindows()
        "$libname.dll"
    else
        error("Not supported: $(Sys.KERNEL)")
    end
end

dylib = dylib_filenames()
const slintwrapper = joinpath(@__DIR__, dylib)

function check_deps()
    global slintwrapper
    if !isfile(slintwrapper)
        error("$slintwrapper does not exist, Please re-run ENV[\"JULIA_SLINT_REBUILD\"]=1;Pkg.build(\"Slint\"), and restart Julia.")
    end
    handle = Libdl.dlopen_e(slintwrapper)
    if handle == C_NULL
        error("$slintwrapper cannot be opened, Please re-run ENV[\"JULIA_SLINT_REBUILD\"]=1;Pkg.build(\"Slint\"), and restart Julia.")
    end
    return handle
end
