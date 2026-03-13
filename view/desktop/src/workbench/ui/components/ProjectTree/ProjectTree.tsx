import { WorkspaceMode } from "@repo/base";

import { ProjectTreeContext } from "./ProjectTreeContext.tsx";
import { TreeRoot } from "./TreeRoot/TreeRoot.tsx";
import { ProjectTreeRoot } from "./types.ts";
import { checkIfAllFoldersAreCollapsed, checkIfAllFoldersAreExpanded } from "./utils/TreeRoot.ts";

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
  showOrders = false,
  showTreeRootIds = false,
}: ProjectTreeProps) => {
  return (
    <div>
      <ProjectTreeContext.Provider
        value={{
          id: tree.id,
          name: tree.name,
          order: tree.order ?? 0,
          iconPath: tree.iconPath,

          allFoldersAreExpanded: checkIfAllFoldersAreExpanded(tree),
          allFoldersAreCollapsed: checkIfAllFoldersAreCollapsed(tree),

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
