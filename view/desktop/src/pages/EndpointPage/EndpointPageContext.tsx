import { createContext } from "react";

import { DescribeEntryOutput, StreamEntriesEvent } from "@repo/moss-project";

interface EndpointPageContext {
  projectId: string;
  entry: StreamEntriesEvent;
  entryDescription: DescribeEntryOutput;
}

export const EndpointPageContext = createContext<EndpointPageContext>({
  projectId: "",
  entry: {
    id: "",
    name: "",
    path: {
      segments: [],
      raw: "",
    },
    class: "component",
    kind: "Item",
    protocol: "Get",
    expanded: false,
  },
  entryDescription: {
    name: "",
    class: "component",
    kind: "Item",
    protocol: "Get",
    url: "",
    headers: [],
    pathParams: [],
    queryParams: [],
  },
});
