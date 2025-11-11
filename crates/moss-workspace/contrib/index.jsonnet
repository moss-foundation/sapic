local contrib = import "../../../contrib/index.libsonnet";

contrib.Register(
  configurations = [
    contrib.configuration.Configuration(
      id = "workspace",
      parent_id = null,
      order = 1,
      name = "Workspace",
      description = null,

      # FIXME: panic if `parameters` are empty
      # message: None, details: "missing field `parameters` at line 6 column 3",
      parameters = [
        contrib.configuration.Parameter(
          id = "<dummy>",
          default = "anyvalue",
          type = contrib.configuration.ParameterType.String,
        ),
      ],
    )
  ],
)


