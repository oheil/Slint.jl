# Note that this script can accept some limited command-line arguments, run
# `julia build_tarballs.jl --help` to see a usage message.
using BinaryBuilder

name = "libslintwrapper"
version = v"0.1.7"

# Collection of sources required to complete build
sources = [
    GitSource("https://github.com/oheil/Slint.jl.git", "83ac1ad0db4bfd2a5b7c3869f51c4d3a798c8822")
]

# Adapted from the justfile of the repo
script = raw"""



"""

# These are the platforms we will build for by default, unless further
# platforms are passed in on the command line
platforms = [
    Platform("x86_64", "macos"; ),
    Platform("x86_64", "linux"; libc = "glibc"),
    Platform("x86_64", "windows")
]

# The products that we will ensure are always built
products = [
    LibraryProduct("libslintwrapper", :libslintwrapper),
    FileProduct("deps/SlintWrapper/include/slintwrapper.h", :slintwrapper_h),
]

# Dependencies that must be installed before this package can be built
dependencies = Dependency[
]

# Build the tarballs, and possibly a `build.jl` as well.
build_tarballs(ARGS, name, version, sources, script, platforms, products, dependencies; compilers=[:c, :rust])
