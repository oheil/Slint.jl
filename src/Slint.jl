module Slint

using CEnum

const deps_file = joinpath(dirname(@__FILE__), "..", "deps", "deps.jl")
if !isfile(deps_file)
    error(raw"Slint.jl is not installed properly, run Pkg.build(\"Slint\") and restart Julia.")
end
include(deps_file)

handle = C_NULL
function __init__()
    global handle
    if handle == C_NULL
        handle = check_deps()
        r_init()
    end
end

function check_init()
    global slintwrapper
    if handle == C_NULL
        __init__()
    end
    if handle == C_NULL
        error("can't load $slintwrapper, Please re-run Pkg.build(\"Slint\"), and restart Julia.")
    end
    return
end

function close()
    global handle
    r = Libdl.dlclose(handle)
    while in(slintwrapper,Libdl.dllist())
        r = Libdl.dlclose(handle)
    end
    handle = C_NULL
    return
end

include("api.jl")

const rMagic = r_get_magic()

# globals for persistance of string return values of callbacks
@enum Rtype rUnknown=1 rBool rString rInteger rFloat
rtypes = ["Unknown","Bool","String","Integer","Float"]
rstring_value = [""]
#struct JRvalue
#   magic::Int32
#   rtype::Ptr{Cchar}
#   int_value::Int32
#   string_value::Ptr{Cchar}
#end
function JRvalue(v::Int)
    JRvalue(
        Cint(rMagic),
        Base.unsafe_convert(Cstring,rtypes[Int(rInteger)]),
        Cint(v),
        Cdouble(0.0),
        Base.unsafe_convert(Cstring,"")
    )
end
function JRvalue(v::Bool)
    JRvalue(
        Cint(rMagic),
        Base.unsafe_convert(Cstring,rtypes[Int(rBool)]),
        Cint(v),
        Cdouble(0.0),
        Base.unsafe_convert(Cstring,"")
    )
end
function JRvalue(v::String)
    JRvalue(
        Cint(rMagic),
        Base.unsafe_convert(Cstring,rtypes[Int(rString)]),
        Cint(-1),
        Cdouble(0.0),
        Base.unsafe_convert(Cstring,v)
    )
end
function JRvalue(v::Float64)
    JRvalue(
        Cint(rMagic),
        Base.unsafe_convert(Cstring,rtypes[Int(rFloat)]),
        Cint(-1),
        Cdouble(v),
        Base.unsafe_convert(Cstring,"")
    )
end


function run()
    check_init()
    r_run()
end

@doc raw"""
See also: `Slint.compile_from_string(slint_string::AbstractString)`
# `Slint.compile_from_file(slint_file::AbstractString, start_comp::AbstractString)`
Compile a `.slint` file, which contains your user interface, written in the Slint language.

## Parameter:
    slint_file::AbstractString

The path and file name of your `.slint` file.

    start_comp::AbstractString

The start component to run

## Return value:
    none

## Example:
```
using Slint
slintFile = "examples\\7guis\\booker.slint"
Slint.compile_from_file(slintFile)
Slint.run()
```
"""
function compile_from_file(slint_file::AbstractString, start_comp::AbstractString)
    check_init()
    r_compile_from_file(slint_file, start_comp)
end

@doc raw"""
See also: `Slint.compile_from_file(slint_file::AbstractString)`
# `Slint.compile_from_string(slint_string::AbstractString)`
Compile a string, which contains your user interface, written in the Slint language.

## Parameter:
slint_string::AbstractString

The string containing your user interface, written in the Slint language.

## Return value:
    none

## Example:
```julia
using Slint
s = "export component MyWin inherits Window {
        Text {
            text: \\"Hello, World\\";
        }
    }
    "
Slint.compile_from_string(s)
Slint.run()
```
"""
function compile_from_string(slint_string::AbstractString, start_comp::AbstractString)
    check_init()
    r_compile_from_string(slint_string, start_comp)
end

#
# remove a row at index from model
#  
function remove_row(id, index)
    check_init()
    r_remove_row(id, index)
    nothing
end

#
# push a full row into model
#  
function push_row(id, new_values)
    check_init()
    nvalues = Vector{JRvalue}()
    len = length(new_values)
    for nv in new_values
        nvalue=JRvalue(nv)
        push!(nvalues,nvalue)
    end
    r_push_row(id, nvalues, len)
    nothing
end

#
# set the value of a property 
#  
function set_value(id, new_value)
    check_init()
    rv = JRvalue(new_value)
    r_set_value(id, rv)
end

#
# get the value of a element/cell as string
#  
function get_cell_value(id, row, col)
    check_init()
    rv = r_get_cell_value(id,row,col)
    if rv.magic == Cint(rMagic)
        return unsafe_string(rv.string_value)
    end
    return ""
end

#
# get the value of a element/cell as Float64
#  
function get_cell_value(::Type{T}, id, row, col) where T<:Float64
    check_init()
    rv = r_get_cell_value(id,row,col)
    rtype=unsafe_string(rv.rtype)
    if rv.magic == Cint(rMagic) && 
        ( rtype == rtypes[Int(rUnknown)] || rtype == rtypes[Int(rFloat)] )
        return rv.float_value
    end
    error(raw"Slint.get_cell_value: return value is not a Float as expected")
    return 0.0
end

#
# get the value of a element/cell as Int
#  
function get_cell_value(::Type{T}, id, row, col) where T<:Int
    check_init()
    rv = r_get_cell_value(id,row,col)
    rtype=unsafe_string(rv.rtype)
    if rv.magic == Cint(rMagic) && 
        ( rtype == rtypes[Int(rUnknown)] || rtype == rtypes[Int(rInteger)] )
        return rv.int_value
    end
    error(raw"Slint.get_cell_value: return value is not an Int as expected")
    return 0
end

#
# get the value of a element/cell as Float64
#  
function get_cell_value(::Type{T}, id, row, col) where T<:String
    check_init()
    rv = r_get_cell_value(id,row,col)
    rtype=unsafe_string(rv.rtype)
    if rv.magic == Cint(rMagic) && 
        ( rtype == rtypes[Int(rUnknown)] || rtype == rtypes[Int(rString)] )
        return unsafe_string(rv.string_value)
    end
    error(raw"Slint.get_cell_value: return value is not a String as expected")
    return ""
end

#
# set the string value of a element/cell
#  
function set_cell_value(id, row, col, new_value)
    check_init()
    rv = JRvalue(new_value)
    #r_set_cell_value(id, row, col, new_value)
    r_set_cell_value(id, row, col, rv)
end

#
# set the wrapped users callback function for Slint vectors
#   - the user_callback is called whenever the value of a element/cell has changed
#   - the user_callback receives the tuple (row,col,new_value,old_value) as params...
#       row is the 0-based index as Float64 of the changed element/cell
#       col is the 0-based index as Float64 of the changed element/cell
#       new_value is the new value of the element/cell
#       old_value is the old value of the element/cell
#
function set_property_model(id, rows, cols, user_callback)
    check_init()

    # create the wrapper callback
    c_callback_wrapper = create_callback_wrapper(user_callback)

    # register the complete callback_wrapper for the callback id. This calls into rust (lib.rs:r_set_property_model)
    r_set_property_model(id,rows,cols,c_callback_wrapper)
end

#
# set the wrapped users callback function
#
function set_callback(id, user_callback)
    check_init()

    # create the wrapper callback
    c_callback_wrapper = create_callback_wrapper(user_callback)

    # register the complete callback_wrapper for the callback id. This calls into rust (lib.rs:r_set_callback)
    r_set_callback(id,c_callback_wrapper)
end

#
# create a wrapper function around the users callback function
#  - the users callback functions handles the needed logic of the GUI callback
#  - the wrapper handles parameters to and return values from the users callback
#  =>
#  - the users callback allways receives a tuple with all parameters of different types
#    in the order the Slint GUI provides them
#  - the users callback must return a value of the proper type the Slint gui expects
#  - the wrapper than returns a JRvalue struct to rust, where member JRvalue.type and corresponding
#    member JRvalue.value... is set properly
#  Pseudocode:
#    function callback_wrapper(parameters,pcount)
#       tuple = (parameters...)
#       r = user_callback(tuple...)
#       typeof(r) == Bool ? return JRvalue(type="Bool",valueBool=r)
#       typeof(r) == String ? return JRvalue(type="String",valueString=r)
#       ...
#       return JRvalue(type="Unknown")
#    end
# 
function create_callback_wrapper(user_callback)
    # callback_wrapper function as expression
    #   ptr: a ptr to the rust callback parameters &[Value] 
    #   len: the count of the arguments
    #     from this information the Julia parameter tuple is constructed
    exprfunc = :( func = ( ptr, len ) -> 
        begin

            #println("callback_wrapper")
            #println(ptr)
            #println(len)
            
            # construct the tuple of parameters to call the users callback with:
            params = let t = ()
                for i in 0:(len-1)
                    type = unsafe_string(Slint.r_get_value_type(ptr,len,i))
                    #println(type)
                    if type == "String"
                        p = unsafe_string(Slint.r_get_value_string(ptr,len,i))
                        #println(p)
                        t = (t..., p)
                    elseif type == "Number"
                        p = Slint.r_get_value_number(ptr,len,i,NaN)
                        t = (t..., p)
                    else
                        # argument type is not implemented, discard it
                    end
                end
                t
            end

            #println("params:")
            #println(params)

            # call the users callback and retrieve the result r:
            r = $user_callback(params...)

            # from users callback result create the result struct rv::JRvalue passed back to rust:
            #   all strings need to be global to outlive any garbage collection
            #   (unsure if this is really needed, it works with local strings too)
            if typeof(r) == Bool
                rstring_value[1] = ""
                rv = JRvalue(Cint(rMagic),Base.unsafe_convert(Cstring,rtypes[Int(rBool)]),Cint(r),Cdouble(0.0),Base.unsafe_convert(Cstring,rstring_value[1]))
                #rptr = Ptr{Cvoid}(pointer_from_objref(Ref(test_rv)))
                #return rptr
            elseif typeof(r) == Int
                rstring_value[1] = ""
                rv = JRvalue(Cint(rMagic),Base.unsafe_convert(Cstring,rtypes[Int(rInteger)]),Cint(r),Cdouble(0.0),Base.unsafe_convert(Cstring,rstring_value[1]))
            elseif typeof(r) == Float64
                rstring_value[1] = ""
                rv = JRvalue(Cint(rMagic),Base.unsafe_convert(Cstring,rtypes[Int(rFloat)]),Cint(-1),Cdouble(r),Base.unsafe_convert(Cstring,rstring_value[1]))
            else
                # The users return value is not implemented to be passed back to rust, return something empty:
                rstring_value[1] = ""
                rv = JRvalue(Cint(rMagic),Base.unsafe_convert(Cstring,rtypes[Int(rUnknown)]),Cint(-1),Cdouble(0.0),Base.unsafe_convert(Cstring,rstring_value[1]))
            end

            # return to rust
            return rv
        end
    )
    # eval the callback_wrapper function expression to make it a real function
    callback_wrapper = eval(exprfunc)
    # create the expression to generate a C-callable function pointer (like @cfunction but return the expression and not yet the function pointer )
    expr = c_function(callback_wrapper, JRvalue, :( (Ptr{Cvoid},Cint) ) )
    # eval the expression and get the C-callable function pointer, return it
    eval(expr)
end

# 
# adapted from Base c.jl:64 @cfunction
#   to return an expression for explicit eval and to allow non-literal ReturnType and ArgumentTypes...
#   example:
#      rtype = :Cint
#      ptypes = :( (Cstring,) )
#      expr = c_function(callback, rtype, :( $ptypes ) )
#      c_callback = eval(expr)
#   this would allow the user to pass the types as expressions to the API call SetCallback
#   (see below in SetCallback_specific function)
#   
#   With SetCallback (see above) only the explicit eval is needed to be able to create
#   a wrapper around the users callback. This wrapper code handles now the callback argument
#   parameters and the return values automatically.
#
function c_function(f, rt, at)
    if !(isa(at, Expr) && at.head === :tuple)
        throw(ArgumentError("c_function argument types must be a literal tuple"))
    end
    at.head = :call
    pushfirst!(at.args, GlobalRef(Core, :svec))
    if isa(f, Expr) && f.head === :$
        fptr = f.args[1]
        typ = CFunction
    else
        fptr = QuoteNode(f)
        typ = Ptr{Cvoid}
    end
    cfun = Expr(:cfunction, typ, fptr, rt, at, QuoteNode(:ccall))
    return cfun
end

# below code for testing interaction between rust <-> Julia still included:

#struct SharedString
#    handle::Ptr{Nothing}
#end
#function SetCallback_specific(id, callback, rtype, ptype)
#    exprfunc = :( func = (params...) -> begin
#        println("hello")
#        par = let str = ""
#            for param in params
#                println(param)
#                println(typeof(param))
#                if typeof(param) == SharedString
#                    println("special");
#                    str = unsafe_string(Slint.test_conv(param.handle))
#                    println(str)
#                end
#            end
#            str
#        end
#        return $callback(par);
#    end )
#    callback_wrapper = eval(exprfunc)
#    expr = c_function(callback_wrapper, rtype, :( $ptype ))
#    c_callback = eval(expr)
#    Slint.set_callback_specific(id,c_callback)
#end

end # module Slint
