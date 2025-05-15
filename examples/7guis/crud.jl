using Slint

slintFile = "examples\\7guis\\crud.slint"

function on_entry(params...)
end

Slint.compile_from_file(slintFile,"MainWindow")

Slint.set_property_model("names-list-bridge",1,1,on_entry)
entry1="1:Emil, Hans"
entry2="2:Hans, Emil"
Slint.push_row("names-list-bridge",[entry1,entry2])
entry3="3:Hans2, Emil2"
Slint.push_row("names-list-bridge",[entry3])

Slint.run()

