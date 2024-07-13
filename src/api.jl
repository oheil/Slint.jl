using CEnum

struct JRvalue
    magic::Int32
    rtype::Ptr{Cchar}
    int_value::Int32
    string_value::Ptr{Cchar}
end

function r_init()
    ccall((:r_init, slintwrapper), Cvoid, ())
end

function r_compile_from_file(slint_file)
    ccall((:r_compile_from_file, slintwrapper), Cvoid, (Ptr{Cchar},), slint_file)
end

function r_compile_from_string(slint_string)
    ccall((:r_compile_from_string, slintwrapper), Cvoid, (Ptr{Cchar},), slint_string)
end

function r_get_magic()
    ccall((:r_get_magic, slintwrapper), Int32, ())
end

function r_set_callback(id, func)
    ccall((:r_set_callback, slintwrapper), Cvoid, (Ptr{Cchar}, Ptr{Cvoid}), id, func)
end

function r_run()
    ccall((:r_run, slintwrapper), Cvoid, ())
end

function r_get_value_type(args_ptr, len, index)
    ccall((:r_get_value_type, slintwrapper), Ptr{Cchar}, (Ptr{Cvoid}, Int32, Int32), args_ptr, len, index)
end

function r_get_value_string(args_ptr, len, index)
    ccall((:r_get_value_string, slintwrapper), Ptr{Cchar}, (Ptr{Cvoid}, Int32, Int32), args_ptr, len, index)
end

function r_get_value_number(args_ptr, len, index, nan)
    ccall((:r_get_value_number, slintwrapper), Cdouble, (Ptr{Cvoid}, Int32, Int32, Cdouble), args_ptr, len, index, nan)
end

function r_push_row(id, new_values, len)
    ccall((:r_push_row, slintwrapper), Cvoid, (Ptr{Cchar}, Ptr{JRvalue}, Csize_t), id, new_values, len)
end

function r_set_cell_value(id, row, col, new_value)
    ccall((:r_set_cell_value, slintwrapper), Cvoid, (Ptr{Cchar}, Int32, Int32, JRvalue), id, row, col, new_value)
end

function r_get_cell_value(id, row, col)
    ccall((:r_get_cell_value, slintwrapper), JRvalue, (Ptr{Cchar}, Int32, Int32), id, row, col)
end

function r_set_property_model(id, rows, cols, func)
    ccall((:r_set_property_model, slintwrapper), Cvoid, (Ptr{Cchar}, Int32, Int32, Ptr{Cvoid}), id, rows, cols, func)
end

