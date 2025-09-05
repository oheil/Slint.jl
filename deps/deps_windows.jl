import Libdl

const slintwrapper = joinpath(@__DIR__, "slintwrapper.dll")

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
