using Slint

slintFile = "examples\\7guis\\crud.slint"

function on_entry(params...)
end

Slint.compile_from_file(slintFile,"MainWindow")

Slint.set_property_model("names-list",1,1,on_entry)

#entry="Emil, Hans"
#Slint.push_row("names-list",[entry])


Slint.run()

