using Slint

slintFile = "examples/7guis/cells.slint"
startComponent = "MainWindow"

Slint.compile_from_file(slintFile, startComponent)

rows = 10
columns = 5

# implementation of callback for element changes in cell model:
#       in property <[[SlintValue]]> cells;
function on_changed_element(params...)
    # model callbacks receive the following parameters:
    row = Int(params[1])  # 0-based
    col = Int(params[2])  # 0-based
    new_value = params[3]
    old_value = params[4]

    # print the parameters to the console for general debugging:
    println("on_changed_element")
    for p in params
        println(p," ",typeof(p))
    end

    # get the value of a specific cell and print it:
    println( "cell 1,1 value: ", Slint.get_cell_value("cells",1,1) )
    # set the value of a specific cell:
    Slint.set_cell_value("cells",rows,columns,new_value)

    return true
end
# create a model for the cells property and register the callback for element changes:
#       property <[[SlintValue]]> cells;
Slint.set_property_model("cells",rows,columns,on_changed_element) # id,rows,cols,callback

Slint.run()

# unload library
Slint.close()
