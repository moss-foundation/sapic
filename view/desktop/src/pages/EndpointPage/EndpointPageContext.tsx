import { createContext } from "react";

import { DescribeResourceOutput, StreamResourcesEvent } from "@repo/moss-project";

interface EndpointPageContext {
  projectId: string;
  resource: StreamResourcesEvent;
  resourceDescription: DescribeResourceOutput;
}

export const EndpointPageContext = createContext<EndpointPageContext>({
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
