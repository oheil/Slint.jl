import Libdl

const rustlibname = "slintwrapper"
const libname = "lib" * rustlibname

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
        error("$slintwrapper does not exist, Please run ENV[\"JULIA_SLINT_REBUILD\"]=1;Pkg.build(\"Slint\"), and restart Julia.")
    end
    if ! isexecutable(slintwrapper)
        chmod(slintwrapper, filemode(slintwrapper) | 0o755)
    end
    if ! isexecutable(slintwrapper)
        error("$slintwrapper is not executable, Please run ENV[\"JULIA_SLINT_REBUILD\"]=1;Pkg.build(\"Slint\"), and restart Julia.")
    end
    handle = Libdl.dlopen_e(slintwrapper)
    if handle == C_NULL
        error("$slintwrapper cannot be opened, Please run ENV[\"JULIA_SLINT_REBUILD\"]=1;Pkg.build(\"Slint\"), and restart Julia.")
    end
    return handle
end
