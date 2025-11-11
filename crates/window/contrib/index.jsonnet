local contrib = import "../../../contrib/index.libsonnet";

local sidebarPosition = {
  LEFT: "LEFT",
  RIGHT: "RIGHT",
};
local activityBarPosition = {
  DEFAULT: "DEFAULT",
  TOP: "TOP",
  BOTTOM: "BOTTOM"
};

contrib.Register(
  configurations = [
    contrib.configuration.Configuration(
      id = "app",
      parent_id = null,
      order = 1,
      name = "App",
      description = null,
      parameters = [
        contrib.configuration.Parameter(
          id = "colorTheme",
          default = "moss.sapic-theme.lightDefault",
          description = "Specifies the color theme used in the app.",
          type = contrib.configuration.ParameterType.String,
        ),
        contrib.configuration.Parameter(
          id = "language",
          default = "en",
          type = contrib.configuration.ParameterType.String,
        ),
        contrib.configuration.Parameter(
          id = "activityBarPosition",
          default = activityBarPosition.DEFAULT,
          enum = std.objectValues(activityBarPosition),
          type = contrib.configuration.ParameterType.String,
        ),
        contrib.configuration.Parameter(
          id = "sideBarPosition",
          default = sidebarPosition.LEFT,
          enum = std.objectValues(sidebarPosition),
          type = contrib.configuration.ParameterType.String,
        ),
      ],
    )
  ],
)