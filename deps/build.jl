
ENV["CARGO_TARGET_DIR"] = @__DIR__

const rustprojname = "SlintWrapper"
const rustlibname = "slintwrapper"
const juliapackage = "Slint"

# Windows .dlls do not have the "lib" prefix
const libname = Sys.iswindows() ? rustlibname : "lib" * rustlibname

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

function build_dylib()
    dylib = dylib_filename()
    clean(dylib)

    run(Cmd(`cargo build --release`, dir=joinpath(@__DIR__, rustprojname)))

    release_dir = joinpath(@__DIR__, "release")

    release_dylib_filepath = joinpath(release_dir, dylib)
    @assert isfile(release_dylib_filepath) "$release_dylib_filepath not found. Build may have failed."
    mv(release_dylib_filepath, joinpath(@__DIR__, dylib))
    #rm(release_dir, recursive=true)
end

function is_exec()
    dylib = dylib_filename()
    dylib_file = joinpath(@__DIR__, dylib)
    if isfile(dylib_file)
        chmod(dylib_file, filemode(dylib_file) | 0o755)
        if ! isexecutable(dylib_file)
            error("Can't load $dylib, file is not executable, Please run ENV[\"JULIA_SLINT_REBUILD\"]=1;Pkg.build(\"Slint\"), and restart Julia.")
        end
    else
        error("Can't load $dylib, file is missing, Please run ENV[\"JULIA_SLINT_REBUILD\"]=1;Pkg.build(\"Slint\"), and restart Julia.")
    end
end

function clean(dylib)
    #release_dir = joinpath(@__DIR__, "release")
    #isdir(release_dir) && rm(release_dir, recursive=true)

    dylib_file = joinpath(@__DIR__, dylib)
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
is_exec()


