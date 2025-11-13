import { createContext } from "react";

import { DescribeResourceOutput, StreamResourcesEvent } from "@repo/moss-project";

interface EndpointViewContext {
  projectId: string;
  resource: StreamResourcesEvent;
  resourceDescription: DescribeResourceOutput;
}

export const EndpointViewContext = createContext<EndpointViewContext>({
  projectId: "",
  resource: {
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
  resourceDescription: {
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
