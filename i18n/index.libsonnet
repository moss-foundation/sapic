{   
    Namespace: {
        // The main namespace of the application.
        // It contains all the tokens the application needs after loading.
        Main: "main",

        // The bootstrap namespace of the application.
        // It contains all the tokens the application needs after loading.
        //
        // Use cases:
        // - The loading screen (when the application is loading)
        Bootstrap: "bootstrap",
    },
    Translation(tokens = {})::
        assert std.member(["object"], std.type(tokens)) : "tokens must be object";
        assert std.all([std.member(["string"], std.type(tokens[k])) for k in std.objectFields(tokens)]) : "tokens must be object of strings";

        tokens,
} 