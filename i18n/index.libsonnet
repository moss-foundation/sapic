{   
    Namespace: {
        Main: "main",
    },
    Translation(tokens = {})::
        assert std.member(["object"], std.type(tokens)) : "tokens must be object";
        assert std.all([std.member(["string"], std.type(tokens[k])) for k in std.objectFields(tokens)]) : "tokens must be object of strings";

        tokens,
} 