using Slint

#using GLMakie
#using GLMakie.Colors
#using Plots, FileIO
#using PyPlot
#using GLMakie.Colors

slintFile = "examples\\plotter\\plotter.slint"
startComponent = "MainWindow"

Slint.compile_from_file(slintFile,startComponent)

#=
#
# This example uses PyPlot to render a 3D surface plot.
#   performance is not good enough for interactive use, but it works.
#
using PyPlot, Colors
buffer = zeros(UInt32, 800, 600)
function render_surface!(buffer, elevation, azimuthal, amplitude)
    PyPlot.ioff()
    fig = figure(figsize=(8, 6), dpi=100)
    ax = fig.add_subplot(111, projection="3d")
    ax.view_init(elev=elevation, azim=azimuthal)

    # Make data.
    X = collect(range(-5, 5, 100))
    Y = collect(range(-5, 5, 100))
    R = [ amplitude*sqrt(x^2 + y^2) for x in X for y in Y ]
    Z = reshape(sin.(R),100,100)
    plot_surface(X, Y, Z, cmap=:coolwarm, linewidth=0, antialiased=false)

    fig.canvas.draw()
    buf=fig.canvas.renderer.buffer_rgba()

    g = ( buf[i,j,:] for i in axes(buf,1) for j in axes(buf,2) )
    b = getindex.(reinterpret.(UInt32,g),1)
    buffer .= reshape(b,(size(buf,2),size(buf,1)))
end
=#

#=
#
# This example uses Plots.jl to render a 3D surface plot.
#   performance is not good enough for interactive use, but it works.
#
using Plots, FileIO
buffer = zeros(ARGB32, 800, 600)  # 4 x 800 x 600 Bytes
buffer4io = zeros(UInt8, 800*600*4)
buffer_rot = zeros(ARGB32, 600, 800)
function render_surface!(buffer, elevation, azimuthal, amplitude)
    x = range(-3, 3, length=30)
    p=surface(
        x, x, (x, y)-> amplitude*exp(-x^2 - y^2), c=:viridis, legend=:none,
        nx=50, ny=50, display_option=Plots.GR.OPTION_SHADED_MESH,
        size=(800,600),
        zlims=(-0.5,3.0),
        camera=(azimuthal, elevation),
    );
    io_buf = IOBuffer(buffer4io, read=true, write=true, maxsize=sizeof(buffer4io))
    show(io_buf, MIME("image/png"), p)
    #buffer_rot .= ARGB32.(load(io_buf))
    buffer_rot .= load(io_buf)
    #buffer .= PermutedDimsArray(view(buffer_rot, :, size(buffer_rot, 2):-1:1), (2, 1))
    buffer .= PermutedDimsArray(view(buffer_rot, :, 1:1:size(buffer_rot, 2)), (2, 1))
end
=#

#=
#
# This example uses rust to render a 3D surface plot.
#   performance is good enough for interactive use.
#
buffer = zeros(UInt8, 800, 600, 3) # width=800, height=600, 3 channels (RGB)
function render_surface!(buffer, elevation, azimuthal, amplitude)
    Slint.render_plot_rgb!(buffer, elevation, azimuthal, amplitude)
end
=#

#\=
#
# This example uses rust to render a 3D surface plot.
#   performance is good enough for interactive use.
#
buffer = zeros(UInt8, 800, 600, 4) # width=800, height=600, 4 channels (RGBA)
function render_surface!(buffer, elevation, azimuthal, amplitude)
    Slint.render_plot_rgba!(buffer, elevation, azimuthal, amplitude)
end
#\=#

# implementation of callback:
#       pure callback render_plot(/* pitch */ float, /* yaw */ float, /* amplitude */ float) -> image;
function on_render_plot(params...)
    #for p in params
    #    println(p," ",typeof(p))
    #end
    elevation = 30.0 + 10.0 * params[1]
    azimuthal = 30.0 + 10.0 * params[2]
    amplitude = 1.0 * params[3] / 2.0

    render_surface!(buffer, elevation, azimuthal, amplitude)
    return buffer
end
# register callback for:
#       pure callback render_plot(/* pitch */ float, /* yaw */ float, /* amplitude */ float) -> image;
Slint.set_callback("render_plot", on_render_plot)

Slint.run()
# unload library
Slint.close()
