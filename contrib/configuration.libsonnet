{
  ParameterType: {
    String: "string",
    Number: "number",
    Boolean: "boolean",
    Object: "object",
    Array: "array",
  },

  Parameter(
    id, 
    default = null, 
    typ,
    description = null, 
    maximum = null, 
    minimum = null, 
    excluded = false,
    protected = false,
    order = null,
    tags = [],
  )::
    assert std.type(id) == "string" : "id must be string";
    assert std.type(typ) == "string" : "typ must be string";
    assert std.member(["string", "null"], std.type(default)) : "default must be string or null";
    assert std.member(["string", "null"], std.type(description)) : "description must be string or null";
    assert std.member(["number", "null"], std.type(maximum)) : "maximum must be number or null";
    assert std.member(["number", "null"], std.type(minimum)) : "minimum must be number or null";
    assert std.type(excluded) == "boolean" : "excluded must be boolean";
    assert std.type(protected) == "boolean" : "protected must be boolean";
    assert std.member(["number", "null"], std.type(order)) : "order must be number or null";
    assert std.type(tags) == "array" : "tags must be array";

    std.prune({
      id: id,
      default: default,
      typ: typ,
      description: description,
      maximum: maximum,
      minimum: minimum,
      excluded: excluded,
      protected: protected,
      order: order,
      tags: tags,
    }),
}
