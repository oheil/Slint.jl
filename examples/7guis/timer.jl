using Slint

slintFile = "examples/7guis/timer.slint"
startComponent = "MainWindow"

Slint.compile_from_file(slintFile,startComponent)

Slint.run()
# unload library
Slint.close()
