local contrib = import "../../../contrib/index.libsonnet";

local sidebarLocation = {
  Left: "LEFT",
  Right: "RIGHT",
};

contrib.configuration.Configuration(
  id = "workspace",
  parent_id = null,
  order = 1,
  name = "Workspace",
  description = null,
  parameters = [
    contrib.configuration.Parameter(
      id = "sideBar.location",
      default = sidebarLocation.Left,
      enum = std.objectValues(sidebarLocation),
      type = contrib.configuration.ParameterType.String,
    ),
  ],
)
