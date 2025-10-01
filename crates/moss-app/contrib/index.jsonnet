local contrib = import "../../../contrib/index.libsonnet";

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
      id = "locale",
      default = "moss.sapic-locale.en",
      type = contrib.configuration.ParameterType.String,
    ),
  ],
)
