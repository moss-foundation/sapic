import { WorkspaceMode } from "@repo/base";

import { useTrackAllProjectStates } from "./hooks/useTrackAllProjectStates.ts";
import { ProjectTreeContext } from "./ProjectTreeContext.tsx";
import { TreeRoot } from "./TreeRoot/TreeRoot.tsx";
import { ProjectTreeRoot } from "./types.ts";

interface ProjectTreeProps {
  tree: ProjectTreeRoot;

  treePaddingLeft?: number;
  treePaddingRight?: number;
  nodeOffset?: number;
  searchInput?: string;
  displayMode?: WorkspaceMode;

  showOrders?: boolean;
  showTreeRootIds?: boolean;
}

export const ProjectTree = ({
  tree,
  searchInput,
  displayMode = "LIVE",
  showOrders = true,
  showTreeRootIds = false,
}: ProjectTreeProps) => {
  const { isFullyExpanded, isFullyCollapsed } = useTrackAllProjectStates(tree);

  return (
    <div>
      <ProjectTreeContext.Provider
        value={{
          id: tree.id,
          name: tree.name,
          order: tree.order ?? 0,
          iconPath: tree.iconPath,

          isFullyExpanded,
          isFullyCollapsed,

          searchInput: searchInput ?? "",

          displayMode,

          showOrders,
          showTreeRootIds,
        }}
      >
        <TreeRoot tree={tree} />
      </ProjectTreeContext.Provider>
    </div>
  );
};

export default ProjectTree;
