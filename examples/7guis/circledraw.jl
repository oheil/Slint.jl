using Slint

slintFile = "examples\\7guis\\circledraw.slint"

Slint.compile_from_file(slintFile,"MainWindow")

function on_undo_clicked(params...)
    println("on_undo_clicked")
    for p in params
        println(p," ",typeof(p))
    end
    return true
end
Slint.set_callback("undo_clicked", on_undo_clicked)

function on_redo_clicked(params...)
    println("on_redo_clicked")
    for p in params
        println(p," ",typeof(p))
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
    Slint.push_row("model",[x,y,30])
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
    Slint.set_cell_value("model",row,col,value)
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