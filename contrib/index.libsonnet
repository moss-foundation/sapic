local modules = {
  configuration: import "configuration.libsonnet",
  resource: import "resource.libsonnet",
};

local types = {
    ContributionPoint: {
        Configuration: "configurations",
        ResourceStatuses: "resourceStatuses",
        HttpHeaders: "httpHeaders"
    },

    Register(
        configurations = [],
        resourceStatuses = [],
        httpHeaders = [],
    )::
        assert std.all([x.__kind__ == "Configuration" for x in configurations]) : "configurations must be array of Configuration";
        assert std.all([x.__kind__ == "ResourceStatus" for x in resourceStatuses]) : "resourceStatuses must be array of ResourceStatus";
        assert std.all([x.__kind__ == "HttpHeader" for x in httpHeaders]) : "httpHeaders must be array of HttpHeader";

        std.prune({
            configurations: configurations,
            resourceStatuses: resourceStatuses,
            httpHeaders: httpHeaders,
        }),
};

{
    data: {
        version: "1.0",
    },
} + types + modules