{
    Status(
        name,
        description = null,
        // This parameter determines the color of the status in the UI.
        color = "#6C707E",
    )::
        assert std.member(["string"], std.type(name)) : "name must be string";
        assert std.member(["string", "null"], std.type(description)) : "description must be string or null";
        assert std.member(["string"], std.type(color)) : "color must be string";

        local this = {
            name: name,
            description: description,
            color: color,
        };

        this + {
            __kind__:: "Status"
        },

    Header(
        name,
        description = null,
        value,
        // This parameter determines whether the header can be disabled/modified by the user.
        protected = false,
        // This parameter determines whether this header will be enabled by default for all endpoints.
        disabled = true,
        order = null,
    )::
        assert std.member(["string"], std.type(name)) : "name must be string";
        assert std.member(["string", "null"], std.type(description)) : "description must be string or null";
        assert std.member(["string"], std.type(value)) : "value must be string";
        assert std.member(["boolean"], std.type(protected)) : "protected must be boolean";
        assert std.member(["number", "null"], std.type(order)) : "order must be number or null";
        
        local this = {
            name: name,
            description: description,
            value: value,
            protected: protected,
        };

        this + {
            __kind__:: "Header"
        },

    ResourceParams(
        statuses = [],
        headers = [],
    )::
        assert std.all([x.__kind__ == "Status" for x in statuses]) : "statuses must be array of Status";
        assert std.all([x.__kind__ == "Header" for x in headers]) : "headers must be array of Header";

        local this = {
            statuses: statuses,
            headers: headers,
        };

        this + {
            __kind__:: "ResourceParams"
        },
}