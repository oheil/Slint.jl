using Slint

using GLMakie
using GLMakie.Colors

slintFile = "examples\\plotter\\plotter.slint"
startComponent = "MainWindow"

Slint.compile_from_file(slintFile,startComponent)

buffer = zeros(RGB24, 600, 800)   # 4 x w=800 x h=600 Bytes, sizeof(buffer)
#buffer = zeros(ARGB32, 800, 600)  # 4 x 800 x 600 Bytes
#buffer = zeros(UInt32, 800, 600)   # 4 x 800 x 600 Bytes

GLMakie.activate!(; visible = false)
#fig = GLMakie.Figure(resolution = (800, 600), backgroundcolor = :black)
#scr = GLMakie.Screen(fig.scene; start_renderloop=false)
#fig = GLMakie.Figure(size = (800, 600))
#scr = GLMakie.Screen(fig.scene; start_renderloop=false)

function initFig()
    fig = GLMakie.Figure(size = (800, 600))
    scr = GLMakie.Screen(fig.scene, Makie.ImageStorageFormat; start_renderloop=false)
    ax = Axis3(fig[1, 1], aspect = :data)
    f = let fig = fig, scr = scr, ax = ax
        () -> (fig, scr, ax)
    end
    return f
end
#GetFig = initFig()

#ax = Axis3(fig[1, 1], aspect = :data)

#pltobj = surface!(ax, x, y, fvalues, color = fargs,
#    colormap = :roma, colorrange = (-π, π),
#    backlight = 1.0f0, highclip = :black);
#resize_to_layout!(fig)

#buffer .= colorbuffer(fig.scene; scr.config)

#fig, ax, pltobj = surface(x, y, fvalues, color = fargs,
#    colormap = :roma, colorrange = (-π, π),
#    backlight = 1.0f0, highclip = :black,
#    figure = (; size = sz, fontsize = 22));

#resize_to_layout!(fig)
#scr = GLMakie.Screen(fig.scene; start_renderloop=false)

# implementation of callback:
#       pure callback render_plot(/* pitch */ float, /* yaw */ float, /* amplitude */ float) -> image;
function on_render_plot(params...)
    for p in params
        println(p," ",typeof(p))
    end

    amplitude = params[3]

    # Makie NOT WORKGING, CairoMakie, GLMakie, WGLMakie all not working!

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
    #   
    #   using ImageView
    #   imshow(img)
    #   
    #   image(myfig[1,1],img,axis = (title = "Default",))  ???

    # Ptr{UInt8}, Ptr{Nothing} ???
    # buffer=zeros(ARGB32, WIDTH, HEIGHT)
    
    #return C_NULL

    #=

    sz = (800, 600)
    px_per_unit = 1
    surf = CairoMakie.Cairo.CairoImageSurface(buffer, CairoMakie.Cairo.FORMAT_ARGB32; flipxy=false)
    conf = Makie.merge_screen_config(CairoMakie.ScreenConfig, Dict(:px_per_unit => px_per_unit))

    x = -2:0.005:2
    y = -2:0.005:2
    f(z) = (amplitude/5.0) * (z^2 + 1) / (z^2 - 1)
    fvals = [f(u + 1im * v) for u in x, v in y]
    fvalues = abs.(fvals)
    fargs = angle.(fvals)
    indxCut = fvalues .> 3
    fvalues[indxCut] .= 3.01
    fig, ax, pltobj = surface(x, y, fvalues, color = fargs,
        colormap = :roma, colorrange = (-π, π),
        backlight = 1.0f0, highclip = :black,
        figure = (; size = sz, fontsize = 22));

    resize_to_layout!(fig)
    scr = CairoMakie.Screen(fig.scene, conf, surf)
    CairoMakie.cairo_draw(scr, fig.scene)

    =#

	sz = (800, 600)

	x = -2:0.005:2
	y = -2:0.005:2
	f(z) = (amplitude/5.0) * (z^2 + 1) / (z^2 - 1)
	fvals = [f(u + 1im * v) for u in x, v in y]
	fvalues = abs.(fvals)
	fargs = angle.(fvals)
	indxCut = fvalues .> 3
	fvalues[indxCut] .= 3.01

    #delete!(ax,pltobj)

    #pltobj = surface!(ax, x, y, fvalues, color = fargs,
    #    colormap = :roma, colorrange = (-π, π),
    #    backlight = 1.0f0, highclip = :black);
	#resize_to_layout!(fig)
    GetFig = initFig()
    fig, scr, ax = GetFig()

    #fig, ax, pltobj = surface(x, y, fvalues, color = fargs,
    #    colormap = :roma, colorrange = (-π, π),
    #    backlight = 1.0f0, highclip = :black,
    #    figure = (; size = sz, fontsize = 22));
    #ax = Axis3(fig[1, 1], aspect = :data)
    pltobj = surface!(ax, x, y, fvalues, color = fargs,
        colormap = :roma, colorrange = (-π, π),
        backlight = 1.0f0, highclip = :black);

    #scr = GLMakie.Screen(fig.scene; start_renderloop=false)

    resize_to_layout!(fig)

    GLMakie.ShaderAbstractions.switch_context!(scr.glscreen)
    ctex = scr.framebuffer.buffers[:color]
    #GLMakie.GLFW.PostEmptyEvent()
    #GLMakie.GLFW.PollEvents()
    #GLMakie.pollevents(scr, Makie.BackendTick)
    GLMakie.render_frame(scr, resize_buffers=false)
    framecache = Matrix{RGB{Makie.N0f8}}(undef, size(ctex))
    GLMakie.fast_color_data!(framecache, ctex)
    buffer .= PermutedDimsArray(view(framecache, :, size(framecache, 2):-1:1), (2, 1))


    #scr = GLMakie.Screen(fig.scene; start_renderloop=false)
    #colorbuffer(fig.scene)
    #buffer .= colorbuffer(fig.scene; scr.config)

    #using ImageView
    #imshow(buffer)
    GLMakie.closeall()

    return buffer
    
end
# register callback for:
#       pure callback render_plot(/* pitch */ float, /* yaw */ float, /* amplitude */ float) -> image;
Slint.set_callback("render_plot", on_render_plot)

Slint.run()
# unload library
Slint.close()
