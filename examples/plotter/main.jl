using Slint

slintFile = "examples\\plotter\\plotter.slint"
startComponent = "MainWindow"

Slint.compile_from_file(slintFile,startComponent)

# implementation of callback:
#       pure callback render_plot(/* pitch */ float, /* yaw */ float, /* amplitude */ float) -> image;
function on_render_plot(params...)
    for p in params
        println(p," ",typeof(p))
    end

    return C_NULL

    
end
# register callback for:
#       pure callback render_plot(/* pitch */ float, /* yaw */ float, /* amplitude */ float) -> image;
Slint.set_callback("render_plot", on_render_plot)

Slint.run()
# unload library
Slint.close()
