package theme

theme_modes := {"dark", "light"}
color_types := {"solid", "gradient", "variable"}

default solid_ok(x) := false

# Hex Color
solid_ok(x) := true if {
	regex.match(`^#([0-9a-fA-F]{3}|[0-9a-fA-F]{4}|[0-9a-fA-F]{6}|[0-9a-fA-F]{8})$`, x)
}

# RGB(A) Color
solid_ok(x) := true if {
	regex.match(`^rgba?\(\s*\d{1,3}\s*,\s*\d{1,3}\s*,\s*\d{1,3}\s*(?:,\s*(?:0?\.\d+|[01]))?\s*\)$`, x)
}

# Named Color
solid_ok(x) := true if {
	regex.match(`^[a-z]+$`, x)
}

default gradient_ok(x) := false

gradient_ok(x) := true if {
	regex.match(`^\w+-gradient\(.*?\)\)$|^\w+-gradient\(.*?\)$`, x)
}

default color_ok(color) := false

color_ok(color) := true if {
	color.type == "solid"
    solid_ok(color.value)
}

color_ok(color) := true if {
	color.type == "gradient"
    gradient_ok(color.value)
}

color_ok(color) := true if {
	color.type == "variable"
    # TODO: Variable validation rule?
}

default boxshadow_ok(x) := false

boxshadow_ok(x) := true if {
	# Box shadow doesn't have a simple regex validation rule
    is_string(x)
    x != ""
}


errors contains "must have 'identifier' field" if {
	object.get(input, "identifier", "") == ""
}


errors contains "'identifier' must be a string" if {
	not is_string(input.identifier)
}

errors contains "must have 'displayName' field" if {
	object.get(input, "displayName", "") == ""
}


errors contains "'displayName' must be a string" if {
	not is_string(input.identifier)
}

errors contains "must have 'mode' field" if {
	object.get(input, "mode", "") == ""
}

errors contains "invalid theme mode" if {
	not theme_modes[input.mode]
}

errors contains "must have 'palette' field" if {
	object.get(input, "palette", {}) == {}
}

errors contains "'palette' must be an object" if {
	not is_object(input.palette)
}

errors contains "must have 'colors' field" if {
	object.get(input, "colors", {}) == {}
}

errors contains "'colors' must be an object" if {
	not is_object(input.colors)
}

errors contains "must have 'boxShadows' field" if {
	object.get(input, "boxShadows", {}) == {}
}

errors contains "'boxShadows' must be an object" if {
	not is_object(input.boxShadows)
}

errors contains msg if {
	msg := sprintf("invalid palette color: %s",[key])
	input.palette[key]
    not color_ok(input.palette[key])
}

errors contains msg if {
	msg := sprintf("invalid color: %s",[key])
    input.colors[key]
    not color_ok(input.colors[key])
}

errors contains msg if {
	msg := sprintf("invalid box shadow: %s", [key])
	input.boxShadows[key]
    not boxshadow_ok(input.boxShadows[key])
}
