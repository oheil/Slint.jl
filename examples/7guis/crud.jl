using Slint

slintFile = "examples\\7guis\\crud.slint"

function on_names_list_entry(params...)
    println("on_names_list_entry")
end

Slint.compile_from_file(slintFile,"MainWindow")

Slint.set_property_model("names-list-bridge",1,1,on_names_list_entry)
entry1="Emil, Hans"
entry2="Mustermann, Max"
entry3="Tisch, Roman"
entries = [entry1,entry2,entry3]
Slint.push_row("names-list-bridge",entries)

function on_prefix_edited(params...)
    prefix = Slint.get_value("prefix")

    filtered_entries = filter(e -> startswith(e, prefix), entries)
    Slint.clear_rows("names-list-bridge")
    Slint.push_row("names-list-bridge",filtered_entries)

    return true
end
Slint.set_callback("prefixEdited", on_prefix_edited)

Slint.run()

