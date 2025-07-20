using Slint

slintFile = "examples\\gallery\\gallery.slint"
startComponent = "App"

Slint.compile_from_file(slintFile,startComponent)

Slint.run()
# unload library
Slint.close()
