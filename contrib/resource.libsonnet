{
    ResourceStatus(
        name,
        description = null,
        // This parameter determines the color of the status in the UI.
        color = "#6C707E",
        // This parameter determines the resources that the status is applicable to.
        resources = ["*"],
    )::
        assert std.member(["string"], std.type(name)) : "name must be string";
        assert std.member(["string", "null"], std.type(description)) : "description must be string or null";
        assert std.member(["string"], std.type(color)) : "color must be string";
        assert std.member(["array"], std.type(resources)) : "resources must be array";
        assert std.all([std.member(["string"], std.type(x)) for x in resources]) : "resources must be array of strings";

        local this = {
            name: name,
            description: description,
            color: color,
            resources: resources,
        };

        this + {
            __kind__:: "ResourceStatus"
        },

    HttpHeader(
        name,
        description = null,
        value,
        // This parameter determines whether the header can be disabled/modified by the user.
        protected = false,
        // This parameter determines whether this header will be enabled by default for all endpoints.
        disabled = true,
    )::
        assert std.member(["string"], std.type(name)) : "name must be string";
        assert std.member(["string", "null"], std.type(description)) : "description must be string or null";
        assert std.member(["string"], std.type(value)) : "value must be string";
        assert std.member(["boolean"], std.type(protected)) : "protected must be boolean";
        assert std.member(["boolean"], std.type(disabled)) : "disabled must be boolean";
        
        local this = {
            name: name,
            description: description,
            value: value,
            protected: protected,
            disabled: disabled,
        };

        this + {
            __kind__:: "HttpHeader"
        },
}