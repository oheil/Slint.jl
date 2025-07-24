
ENV["CARGO_TARGET_DIR"] = @__DIR__

const rustprojname = "SlintWrapper"
const rustlibname = "slintwrapper"
const juliapackage = "Slint"

# Windows .dlls do not have the "lib" prefix
const libname = Sys.iswindows() ? rustlibname : "lib" * rustlibname

function build_dylib()
    clean()

    run(Cmd(`cargo build --release`, dir=joinpath(@__DIR__, rustprojname)))

    release_dir = joinpath(@__DIR__, "release")
    dylib = dylib_filename()

    release_dylib_filepath = joinpath(release_dir, dylib)
    @assert isfile(release_dylib_filepath) "$release_dylib_filepath not found. Build may have failed."
    mv(release_dylib_filepath, joinpath(@__DIR__, dylib))
    #rm(release_dir, recursive=true)

    #write_deps_file(libname, dylib, juliapackage)
end


function dylib_filename()
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

function write_deps_file(libname, libfile, juliapackage)
    script = """
import Libdl

const $rustlibname = joinpath(@__DIR__, "$libfile")

function check_deps()
    global $rustlibname
    if !isfile($rustlibname)
        error("\$$rustlibname does not exist, Please re-run Pkg.build(\\"$juliapackage\\"), and restart Julia.")
    end
    handle = Libdl.dlopen_e($rustlibname)
    if handle == C_NULL
        error("\$$rustlibname cannot be opened, Please re-run Pkg.build(\\"$juliapackage\\"), and restart Julia.")
    end
    return handle
end
"""

    open(joinpath(@__DIR__, "deps.jl"), "w") do f
        write(f, script)
    end
end

function clean()
    deps_file = joinpath(@__DIR__, "deps.jl")
    isfile(deps_file) && rm(deps_file)

    #release_dir = joinpath(@__DIR__, "release")
    #isdir(release_dir) && rm(release_dir, recursive=true)

    dylib_file = joinpath(@__DIR__, dylib_filename())
    isfile(dylib_file) && rm(dylib_file)

    # remove deps\SlintWrapper\include\slintwrapper.h in case it can not be removed or
    # overwritten when it is created anew by deps\SlintWrapper\build\build.rs
    dylib_header = joinpath(@__DIR__, rustprojname, "include", rustlibname*".h")
    isfile(dylib_header) && rm(dylib_header, force=true)
	@assert !isfile(dylib_header) "ERROR: could not remove file $dylib_header"

    # remove Slint\deps\release\build\SlintWrapper-* to force rebuild and 
    # creation of deps\SlintWrapper\include\slintwrapper.h
    # this is a workaround against build errors which occur if you do this on Windows:
    #   pkg> activate --temp
    #   julia> using Pkg; Pkg.add(url="https://github.com/oheil/Slint.jl.git")
    # 
    buildpath = joinpath(@__DIR__, "release", "build")
    if isdir(buildpath)
        for folder in filter(x -> contains(x,"SlintWrapper-"), readdir(buildpath))
            rm(joinpath(buildpath, folder), recursive=true, force=true)
        end
    end

end

if get(ENV, "JULIA_SLINT_REBUILD", "0") == "1"
    build_dylib()
end

dylib = dylib_filename()
write_deps_file(libname, dylib, juliapackage)


