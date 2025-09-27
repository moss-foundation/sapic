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
    enum = null,
    enumDescriptions = null,
    type,
    description = null, 
    minLength = null,
    maxLength = null,
    maximum = null, 
    minimum = null, 
    excluded = false,
    protected = false,
    order = null,
    tags = [],
  )::
    assert std.member(["string"], std.type(id)) : "id must be string";
    assert std.member(["string"], std.type(type)) : "typ must be string";
    assert std.member(["string", "null"], std.type(default)) : "default must be string or null";
    assert std.member(["array", "null"], std.type(enum)) : "enum must be array or null";
    assert std.member(["array", "null"], std.type(enumDescriptions)) : "enumDescriptions must be array or null";
    assert enumDescriptions == null || std.all([std.member(["string"], std.type(x)) for x in enumDescriptions]) : "enumDescriptions elements must be strings";
    assert std.member(["string", "null"], std.type(description)) : "description must be string or null";
    assert std.member(["number", "null"], std.type(minLength)) : "minLength must be number or null";
    assert std.member(["number", "null"], std.type(maxLength)) : "maxLength must be number or null";
    assert std.member(["number", "null"], std.type(maximum)) : "maximum must be number or null";
    assert std.member(["number", "null"], std.type(minimum)) : "minimum must be number or null";
    assert std.member(["boolean"], std.type(excluded)) : "excluded must be boolean";
    assert std.member(["boolean"], std.type(protected)) : "protected must be boolean";
    assert std.member(["number", "null"], std.type(order)) : "order must be number or null";
    assert std.member(["array"], std.type(tags)) : "tags must be array";

    local this = {
      id: id,
      default: default,
      enum: enum,
      type: type,
      description: description,
      minLength: minLength,
      maxLength: maxLength,
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

    local this = {
      id: id,
      parent_id: parent_id,
      order: order,
      name: name,
      description: description,
      parameters: parameters,
    };

    std.prune(this + {
      __kind__:: "Configuration"
    }),
}
