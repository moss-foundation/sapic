import { createContext, useContext } from "react";

import { TreeContext } from "@/lib/ui/Tree/components/TreeContext";
import { WorkspaceMode } from "@repo/base";

import { BaseTreeContextProps } from "../../../../lib/ui/Tree/types";

export interface ProjectTreeContextProps extends BaseTreeContextProps {
  iconPath?: string;
  allFoldersAreExpanded: boolean;
  allFoldersAreCollapsed: boolean;
  searchInput: string;
  displayMode: WorkspaceMode;
}

export const ProjectTreeContext = createContext<ProjectTreeContextProps>({
  id: "",
  name: "",
  order: 0,
  iconPath: undefined,
  treePaddingLeft: 0,
  treePaddingRight: 0,
  nodeOffset: 0,
  allFoldersAreExpanded: false,
  allFoldersAreCollapsed: true,
  searchInput: "",
  displayMode: "LIVE",
  showOrders: false,
  showRootNodeIds: false,
});

export const TreeContextBridge = ({ children }: { children: React.ReactNode }) => {
  const projectCtx = useContext(ProjectTreeContext);
  const baseProps: BaseTreeContextProps = {
    id: projectCtx.id,
    name: projectCtx.name,
    order: projectCtx.order,
    treePaddingLeft: projectCtx.treePaddingLeft,
    treePaddingRight: projectCtx.treePaddingRight,
    nodeOffset: projectCtx.nodeOffset,
    showOrders: projectCtx.showOrders,
  };
  return <TreeContext.Provider value={baseProps}>{children}</TreeContext.Provider>;
};
