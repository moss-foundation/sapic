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
    assert std.member(["string"], std.type(id)) : "id must be string";
    assert std.member(["string"], std.type(typ)) : "typ must be string";
    assert std.member(["string", "null"], std.type(default)) : "default must be string or null";
    assert std.member(["string", "null"], std.type(description)) : "description must be string or null";
    assert std.member(["number", "null"], std.type(maximum)) : "maximum must be number or null";
    assert std.member(["number", "null"], std.type(minimum)) : "minimum must be number or null";
    assert std.member(["boolean"], std.type(excluded)) : "excluded must be boolean";
    assert std.member(["boolean"], std.type(protected)) : "protected must be boolean";
    assert std.member(["number", "null"], std.type(order)) : "order must be number or null";
    assert std.member(["array"], std.type(tags)) : "tags must be array";

    local this = {
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
    };
    
    this + {
      __kind__:: "Parameter"
    },


  Configuration(
    id = null,
    parent_id = null,
    order = null,
    name = null,
    description = null,
    parameters = [],
  )::
    assert std.member(["string"], std.type(id)) : "id must be string";
    assert std.member(["string", "null"], std.type(parent_id)) : "parent_id must be string or null";
    assert std.member(["number", "null"], std.type(order)) : "order must be number or null";
    assert std.member(["string", "null"], std.type(name)) : "name must be string or null";
    assert std.member(["string", "null"], std.type(description)) : "description must be string or null";
    assert std.all([x.__kind__ == "Parameter" for x in parameters]) : "parameters must be array of Parameter";

    std.prune({
      id: id,
      parent_id: parent_id,
      order: order,
      name: name,
      description: description,
      parameters: parameters,
    }),
}
