using Slint

slintFile = "examples\\7guis\\circledraw.slint"

Slint.compile_from_file(slintFile,"MainWindow")

@enum Action draw=1 resize=2
let CIRCLE_COUNT = Ref{Int}(0)
    global get_circle_count
    function get_circle_count()
        return CIRCLE_COUNT[]
    end
    global set_circle_count
    function set_circle_count(count)
        CIRCLE_COUNT[] = count
    end
    global inc_circle_count
    function inc_circle_count()
        CIRCLE_COUNT[] += 1
    end
    global dec_circle_count
    function dec_circle_count()
        if CIRCLE_COUNT[]>0
            CIRCLE_COUNT[] -= 1
        end
    end
end
mutable struct Undo
    a::Action
    index::Int
    width::Int
    x::Int
    y::Int
end
UndoStack = Undo[]
RedoStack = Undo[]

function on_undo_clicked(params...)
    #println("on_undo_clicked")
    #for p in params
    #    println(p," ",typeof(p))
    #end
    if get_circle_count() > 0 
        u=pop!(UndoStack)
        Slint.set_value("redoable",true)
        if u.a == draw
            Slint.remove_row("model",get_circle_count())
            dec_circle_count()
        elseif u.a == resize
            row=u.index
            col=3
            old_width=u.width
            new_width=Slint.get_cell_value(Int,"model",row,col)
            Slint.set_cell_value("model",row,col,old_width)
            u.width=new_width
        end
        push!(RedoStack,u)
    end
    if length(UndoStack) == 0
        Slint.set_value("undoable",false)
    end
    return true
end
Slint.set_callback("undo_clicked", on_undo_clicked)

function on_redo_clicked(params...)
    #println("on_redo_clicked")
    #for p in params
    #    println(p," ",typeof(p))
    #end
    i = length(RedoStack)
    if i > 0 
        u=pop!(RedoStack)
        Slint.set_value("undoable",true)
        if u.a == draw
            #println(u.x," ",u.y," ",u.width)
            Slint.push_row("model",[u.x,u.y,u.width])
            inc_circle_count()
        elseif u.a == resize
            row=u.index
            col=3
            new_width=u.width
            old_width=Slint.get_cell_value(Int,"model",row,col)
            Slint.set_cell_value("model",row,col,new_width)
            u.width=old_width
        end
        push!(UndoStack,u)
    end
    if length(RedoStack) == 0
        Slint.set_value("redoable",false)
    end
    return true
end
Slint.set_callback("redo_clicked", on_redo_clicked)

function on_background_clicked(params...)
    #println("on_background_clicked")
    #for p in params
    #    println(p," ",typeof(p))
    #end
    x=Int(params[1])
    y=Int(params[2])
    w=30
    Slint.push_row("model",[x,y,w])
    Slint.set_value("undoable",true)
    inc_circle_count()
    push!(UndoStack,Undo(draw,get_circle_count(),w,x,y))
    return true
end
Slint.set_callback("background_clicked", on_background_clicked)

function on_circle_resized(params...)
    #println("on_circle_resized")
    #for p in params
    #    println(p," ",typeof(p))
    #end
    row=Int(params[1]) + 1
    col=3 
    value=Int(floor(params[2]))
    old_value=Slint.get_cell_value(Int,"model",row,col)
    Slint.set_cell_value("model",row,col,value)
    push!(UndoStack,Undo(resize,row,old_value,0,0))
    return true
end
Slint.set_callback("circle_resized", on_circle_resized)

function on_circle(params...)
    #println("on_draw_circle")
    #for p in params
    #    println(p," ",typeof(p))
    #end

    row = Int(params[1])  # 0-based
    col = Int(params[2])  # 0-based
    new_value = params[3]
    old_value = params[4]

    return true
end
Slint.set_property_model("model",1,3,on_circle) # id,rows,cols,callback


Slint.run()