using CEnum

struct JRvalue
    magic::Int32
    rtype::Ptr{Cchar}
    int_value::Int32
    float_value::Cdouble
    string_value::Ptr{Cchar}
    image_value::Ptr{Cvoid}
    width::Int32
    height::Int32
    elsize::Int32
end

function r_clear_error_state()
    ccall((:r_clear_error_state, slintwrapper), Cvoid, ())
end

function r_get_error_state()
    ccall((:r_get_error_state, slintwrapper), JRvalue, ())
end

function r_init()
    ccall((:r_init, slintwrapper), Cvoid, ())
end

function r_compile_from_file(slint_file, slint_comp)
    ccall((:r_compile_from_file, slintwrapper), Cvoid, (Ptr{Cchar}, Ptr{Cchar}), slint_file, slint_comp)
end

function r_compile_from_string(slint_string, slint_comp)
    ccall((:r_compile_from_string, slintwrapper), Cvoid, (Ptr{Cchar}, Ptr{Cchar}), slint_string, slint_comp)
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

function r_clear_rows(id)
    ccall((:r_clear_rows, slintwrapper), Cvoid, (Ptr{Cchar},), id)
end

function r_remove_row(id, index)
    ccall((:r_remove_row, slintwrapper), Cvoid, (Ptr{Cchar}, Csize_t), id, index)
end

function r_push_rows(id, new_values, len)
    ccall((:r_push_rows, slintwrapper), Cvoid, (Ptr{Cchar}, Ptr{JRvalue}, Csize_t), id, new_values, len)
end

function r_set_value(id, new_value)
    ccall((:r_set_value, slintwrapper), Cvoid, (Ptr{Cchar}, JRvalue), id, new_value)
end

function r_get_value(id)
    ccall((:r_get_value, slintwrapper), JRvalue, (Ptr{Cchar},), id)
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

function r_render_plot_rgb(julia_buffer, pitch, yaw, amplitude)
    ccall((:r_render_plot_rgb, slintwrapper), Cvoid, (JRvalue, Cfloat, Cfloat, Cfloat), julia_buffer, pitch, yaw, amplitude)
end

function r_render_plot_rgba(julia_buffer, pitch, yaw, amplitude)
    ccall((:r_render_plot_rgba, slintwrapper), Cvoid, (JRvalue, Cfloat, Cfloat, Cfloat), julia_buffer, pitch, yaw, amplitude)
end

