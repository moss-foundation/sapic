package theme_test

import data.theme

test_solid_valid_hex_color if {
    theme.solid_ok("#000")        # #RGB
    theme.solid_ok("#0000")       # #RGBA
    theme.solid_ok("#ffffff")     # #RRGGBB
    theme.solid_ok("#FFFFFFFF")   # #RRGGBBAA
}

test_solid_invalid_hex_color if {
    not theme.solid_ok("000")     # Missing hashtag
    not theme.solid_ok("#00000G") # Value out of range
    not theme.solid_ok("#00")     # Invalid length
}

test_solid_valid_rgba_color if {
    theme.solid_ok("rgb(0, 0, 0)")              #RGB
    theme.solid_ok("rgba(255, 255, 255, 0.5)")  #RGBA
}

test_solid_invalid_rgba_color if {
    not theme.solid_ok("abc(0, 0, 0)")    # Invalid color function
    not theme.solid_ok("rgb(0, 0)")       # Invalid argument number
    not theme.solid_ok("rgb(-1, 256, a)") # Argument out of range
}

test_gradient_valid if {
    theme.gradient_ok("linear-gradient(#e66465, #9198e5)")
    theme.gradient_ok("radial-gradient(circle, rgb(255, 0, 0), rgb(0, 0, 255))")
}

test_gradient_invalid if {
    not theme.gradient_ok("abc(#e66465, #9198e5)") # Invalid gradient function
}
