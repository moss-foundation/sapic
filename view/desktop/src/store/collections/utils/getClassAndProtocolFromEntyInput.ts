import { CreateEntryInput } from "@repo/moss-collection";

export const getClassAndProtocolFromEntyInput = (input: CreateEntryInput) => {
  let entryClass: "Request" | "Endpoint" | "Component" | "Schema" = "Request";
  let protocol: "Get" | "Post" | "Put" | "Delete" | "WebSocket" | "Graphql" | "Grpc" | undefined = undefined;

  if ("dir" in input) {
    if ("request" in input.dir.configuration) {
      entryClass = "Request";
    } else if ("endpoint" in input.dir.configuration) {
      entryClass = "Endpoint";
    } else if ("component" in input.dir.configuration) {
      entryClass = "Component";
    } else if ("schema" in input.dir.configuration) {
      entryClass = "Schema";
    }
  }

  if ("item" in input) {
    if ("request" in input.item.configuration) {
      entryClass = "Request";
      if ("http" in input.item.configuration.request) {
        const method = input.item.configuration.request.http.requestParts.method;
        if (method === "GET") protocol = "Get";
        else if (method === "POST") protocol = "Post";
        else if (method === "PUT") protocol = "Put";
        else if (method === "DELETE") protocol = "Delete";
      }
    } else if ("endpoint" in input.item.configuration) {
      entryClass = "Endpoint";
      protocol = "Get";
    } else if ("component" in input.item.configuration) {
      entryClass = "Component";
    } else if ("schema" in input.item.configuration) {
      entryClass = "Schema";
    }
  }

  return {
    entryClass,
    protocol,
  };
};
