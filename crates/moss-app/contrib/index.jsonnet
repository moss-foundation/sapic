local contrib = import "../../../contrib/index.libsonnet";

{
  colorTheme: contrib.configuration.Parameter(
    id = "colorTheme",
    default = "moss.sapic-theme.lightDefault",
    typ = contrib.configuration.ParameterType.String,
  ),
  locale: contrib.configuration.Parameter(
    id = "locale",
    default = "moss.sapic-locale.en",
    typ = contrib.configuration.ParameterType.String,
  ),
}
