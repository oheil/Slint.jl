using Slint

slintFile = "examples\\plotter\\plotter.slint"
startComponent = "MainWindow"

Slint.compile_from_file(slintFile,startComponent)

Slint.run()
# unload library
Slint.close()
