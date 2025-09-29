local modules = {
  configuration: import "configuration.libsonnet",
  resource: import "resource.libsonnet",
};

local types = {
    ContributionPoint: {
        Configuration: "configuration",
        ResourceStatuses: "resourceStatuses",
        HttpHeaders: "httpHeaders"
    },

    Register(
        configuration = null,
        resourceStatuses = [],
        httpHeaders = [],
    )::
        assert configuration == null || configuration.__kind__ == "Configuration" : "configuration must be Configuration or null";
        assert std.all([x.__kind__ == "ResourceStatus" for x in resourceStatuses]) : "resourceStatuses must be array of ResourceStatus";
        assert std.all([x.__kind__ == "HttpHeader" for x in httpHeaders]) : "httpHeaders must be array of HttpHeader";

        std.prune({
            configuration: configuration,
            resourceStatuses: resourceStatuses,
            httpHeaders: httpHeaders,
        }),
};

{
    data: {
        version: "1.0",
    },
} + types + modules