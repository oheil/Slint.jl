using Slint

slintFile = "examples\\7guis\\circledraw.slint"

Slint.compile_from_file(slintFile,"MainWindow")

@enum Action draw=1 resize=2
struct Undo
    a::Action
    index::Int
    width::Int
    x::Int
    y::Int
end
UndoStack = Undo[]
RedoStack = Undo[]

function on_undo_clicked(params...)
    println("on_undo_clicked")
    for p in params
        println(p," ",typeof(p))
    end
    i = length(UndoStack)
    if i > 0 
        u=pop!(UndoStack)
        push!(RedoStack,u)
        Slint.set_value("redoable",true)
        if u.a == draw
            Slint.remove_row("model",i)
        elseif u.a == resize

        end
    end
    i = length(UndoStack)
    if i == 0
        Slint.set_value("undoable",false)
    end
    return true
end
Slint.set_callback("undo_clicked", on_undo_clicked)

function on_redo_clicked(params...)
    println("on_redo_clicked")
    for p in params
        println(p," ",typeof(p))
    end
    i = length(RedoStack)
    if i > 0 
        u=pop!(RedoStack)
        push!(UndoStack,u)
        Slint.set_value("undoable",true)
        if u.a == draw
            println(u.x," ",u.y," ",u.width)
            Slint.push_row("model",[u.x,u.y,u.width])
        elseif u.a == resize

        end
    end
    i = length(RedoStack)
    if i == 0
        Slint.set_value("redoable",false)
    end
    return true
end
Slint.set_callback("redo_clicked", on_redo_clicked)

function on_background_clicked(params...)
    println("on_background_clicked")
    for p in params
        println(p," ",typeof(p))
    end
    x=Int(params[1])
    y=Int(params[2])
    w=30
    Slint.push_row("model",[x,y,w])
    Slint.set_value("undoable",true)
    push!(UndoStack,Undo(draw,0,w,x,y))
    return true
end
Slint.set_callback("background_clicked", on_background_clicked)

function on_circle_resized(params...)
    println("on_circle_resized")
    for p in params
        println(p," ",typeof(p))
    end
    row=Int(params[1]) + 1
    col=3 
    value=Int(floor(params[2]))
    old_value=parse(Float64,Slint.get_cell_value("model",row,col))
    Slint.set_cell_value("model",row,col,value)
    push!(UndoStack,Undo(resize,row,old_value,0,0))
    return true
end
Slint.set_callback("circle_resized", on_circle_resized)

function on_circle(params...)
    row = Int(params[1])  # 0-based
    col = Int(params[2])  # 0-based
    new_value = params[3]
    old_value = params[4]

    println("on_draw_circle")
    for p in params
        println(p," ",typeof(p))
    end

    return true
end
Slint.set_property_model("model",1,3,on_circle) # id,rows,cols,callback


Slint.run()