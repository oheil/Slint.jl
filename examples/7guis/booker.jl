using Slint, Dates

slintFile = "examples\\7guis\\booker.slint"

Slint.compile_from_file(slintFile)

function on_validate_date(params...)
    if isnothing(match(r"^\d\d\.\d\d.\d\d\d\d$",params[1]))
        return false
    end
    return true
end

Slint.set_callback("validate-date", on_validate_date)

function on_compare_date(params...)
    d1 = Dates.tryparse(Date,params[1],dateformat"d.m.Y")
    d2 = Dates.tryparse(Date,params[2],dateformat"d.m.Y")
    if isnothing(d1) || isnothing(d2) || d1 > d2
        return false
    end
    return true
end

Slint.set_callback("compare-date", on_compare_date)

Slint.run()

