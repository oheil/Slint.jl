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

    # https://beautiful.makie.org/dev/examples/3d/surfaces/complex_function
    #   using CairoMakie
    #   CairoMakie.activate!()
    #   x = -2:0.005:2
    #   y = -2:0.005:2
    #   f(z) = (z^2 + 1) / (z^2 - 1)
    #   fvals = [f(u + 1im * v) for u in x, v in y]
    #   fvalues = abs.(fvals)
    #   fargs = angle.(fvals)
    #   indxCut = fvalues .> 3
    #   fvalues[indxCut] .= 3.01
    #   
    #   fig, ax, pltobj = surface(x, y, fvalues, color = fargs,
    #       colormap = :roma, colorrange = (-π, π),
    #       backlight = 1.0f0, highclip = :black,
    #       figure = (; size = (1200, 800), fontsize = 22));
    #   myfig=Figure()  ???
    #   img = colorbuffer(fig.scene);   ???
    #   image(myfig[1,1],img,axis = (title = "Default",))  ???


    return C_NULL

    
end
# register callback for:
#       pure callback render_plot(/* pitch */ float, /* yaw */ float, /* amplitude */ float) -> image;
Slint.set_callback("render_plot", on_render_plot)

Slint.run()
# unload library
Slint.close()
