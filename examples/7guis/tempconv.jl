using Slint

slintFile = "examples/7guis/tempconv.slint"
startComponent = "TempConv"

Slint.compile_from_file(slintFile,startComponent)

Slint.run()
# unload library
Slint.close()
