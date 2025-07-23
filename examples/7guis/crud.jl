using Slint

slintFile = "examples/7guis/crud.slint"
startComponent = "MainWindow"

Slint.compile_from_file(slintFile,startComponent)

# only use the bridge on the Julia side. When filled or changed, it will be used to populate
#   in property <[StandardListViewItem]> names-list;
Slint.set_property_model("names-list-bridge",1,1)

entries = ["Emil, Hans","Mustermann, Max","Tisch, Roman"]
Slint.push_row("names-list-bridge",entries)

filtered_indices = getindex.(collect(enumerate(entries)),1)

# implementation of callback:
#       callback prefixEdited();
function on_prefix_edited(params...)
    prefix = Slint.get_value("prefix")

    global filtered_indices = findall(e -> startswith(e, prefix), entries)
    filtered_entries = entries[filtered_indices]
    Slint.clear_rows("names-list-bridge")
    Slint.push_row("names-list-bridge",filtered_entries)

    return true
end
# register callback for:
#       callback prefixEdited();
Slint.set_callback("prefixEdited", on_prefix_edited)

# implementation of callback:
#       callback updateClicked();
function on_update_clicked(params...)
    current_item = Int(floor(Slint.get_value(Float64,"current-item"))) + 1
    entry_index = filtered_indices[current_item]

    name = Slint.get_value("name")
    surname = Slint.get_value("surname")    

    global entries[entry_index] = "$surname, $name" 
    
    prefix = Slint.get_value("prefix")

    global filtered_indices = findall(e -> startswith(e, prefix), entries)
    filtered_entries = entries[filtered_indices]
    Slint.clear_rows("names-list-bridge")
    Slint.push_row("names-list-bridge",filtered_entries)

    return true
end
# register callback for:
#       callback updateClicked();
Slint.set_callback("updateClicked", on_update_clicked)

# implementation of callback:
#       callback deleteClicked();
function on_delete_clicked(params...)
    current_item = Int(floor(Slint.get_value(Float64,"current-item"))) + 1
    entry_index = filtered_indices[current_item]

    deleteat!(entries, entry_index)
    global filtered_indices = findall(e -> startswith(e, Slint.get_value("prefix")), entries)
    filtered_entries = entries[filtered_indices]
    Slint.clear_rows("names-list-bridge")
    Slint.push_row("names-list-bridge",filtered_entries)

    return true
end
# register callback for:
#       callback deleteClicked();
Slint.set_callback("deleteClicked", on_delete_clicked)

# implementation of callback:
#       callback createClicked();
function on_create_clicked(params...)
    name = Slint.get_value("name")
    surname = Slint.get_value("surname")

    new_entry = "$surname, $name"
    push!(entries, new_entry)

    prefix = Slint.get_value("prefix")
    global filtered_indices = findall(e -> startswith(e, prefix), entries)
    filtered_entries = entries[filtered_indices]
    Slint.clear_rows("names-list-bridge")
    Slint.push_row("names-list-bridge",filtered_entries)

    return true
end
# register callback for:
#       callback createClicked();
Slint.set_callback("createClicked", on_create_clicked)

Slint.run()
# unload library
Slint.close()
