local modules = {
  configuration: import "configuration.libsonnet",
  resource: import "resource.libsonnet",
};

local types = {
    ContributionPoint: {
        Configuration: "configuration",
        Resource: "resource",
    },

    Register(
        configuration = null,
        resource_params = null,
    )::
        assert configuration == null || configuration.__kind__ == "Configuration" : "configuration must be Configuration or null";
        assert resource_params == null || resource_params.__kind__ == "ResourceParams" : "resource_params must be ResourceParams or null";

        std.prune({
            configuration: configuration,
            resource_params: resource_params,
        }),
};

{
    data: {
        version: "1.0",
    },
} + types + modules