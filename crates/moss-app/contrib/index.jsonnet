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
      typ = contrib.configuration.ParameterType.String,
      tags = [],
    ),
    contrib.configuration.Parameter(
      id = "locale",
      default = "moss.sapic-locale.en",
      typ = contrib.configuration.ParameterType.String,
      tags = [],
    ),
  ],
)
