import { CreateEntryInput } from "@repo/moss-collection";

export const getClassAndProtocolFromEntryInput = (input: CreateEntryInput) => {
  let entryClass: "Request" | "Endpoint" | "Component" | "Schema" = "Request";
  let protocol: "Get" | "Post" | "Put" | "Delete" | "WebSocket" | "Graphql" | "Grpc" | undefined = undefined;

  if ("DIR" in input) {
    if ("request" in input.DIR.configuration) {
      entryClass = "Request";
    } else if ("endpoint" in input.DIR.configuration) {
      entryClass = "Endpoint";
    } else if ("component" in input.DIR.configuration) {
      entryClass = "Component";
    } else if ("schema" in input.DIR.configuration) {
      entryClass = "Schema";
    }
  }

  if ("ITEM" in input) {
    if ("request" in input.ITEM.configuration) {
      entryClass = "Request";
      if ("http" in input.ITEM.configuration.request) {
        const method = input.ITEM.configuration.request.http.requestParts.method;
        if (method === "GET") protocol = "Get";
        else if (method === "POST") protocol = "Post";
        else if (method === "PUT") protocol = "Put";
        else if (method === "DELETE") protocol = "Delete";
      }
    } else if ("endpoint" in input.ITEM.configuration) {
      entryClass = "Endpoint";
      protocol = "Get";
    } else if ("component" in input.ITEM.configuration) {
      entryClass = "Component";
    } else if ("schema" in input.ITEM.configuration) {
      entryClass = "Schema";
    }
  }

  return {
    entryClass,
    protocol,
  };
};
