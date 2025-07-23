using Slint

slintFile = "examples/7guis/counter.slint"
startComponent = "Counter"

Slint.compile_from_file(slintFile,startComponent)

Slint.run()
# unload library
Slint.close()
