using Slint

slintFile = "examples\\7guis\\cells.slint"
startComponent = "MainWindow"

Slint.compile_from_file(slintFile, startComponent)

function on_changed_element(params...)
    row = Int(params[1])  # 1-based
    col = Int(params[2])  # 1-based
    new_value = params[3]
    old_value = params[4]

    println("on_changed_element")
    for p in params
        println(p," ",typeof(p))
    end

    println( "cell 1,1 value: ", Slint.get_cell_value("cells",1,1) )
    Slint.set_cell_value("cells",10,5,new_value)

    return true
end
Slint.set_property_model("cells",10,5,on_changed_element) # id,rows,cols,callback

Slint.run()
Slint.close()
