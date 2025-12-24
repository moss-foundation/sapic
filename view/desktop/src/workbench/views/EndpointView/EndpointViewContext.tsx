import { createContext } from "react";

export interface EndpointViewContextProps {
  projectId: string;
  resourceId: string;
}

export const EndpointViewContext = createContext<EndpointViewContextProps>({
  projectId: "",
  resourceId: "",
});
