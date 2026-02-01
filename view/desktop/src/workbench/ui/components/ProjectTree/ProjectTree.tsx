import { ProjectTreeContext } from "./ProjectTreeContext.tsx";
import { TreeRootNode } from "./TreeRootNode/TreeRootNode.tsx";
import { ProjectTreeProps } from "./types.ts";
import { checkIfAllFoldersAreCollapsed, checkIfAllFoldersAreExpanded } from "./utils/TreeRoot.ts";

export const ProjectTree = ({
  tree,
  treePaddingLeft = 8,
  treePaddingRight = 8,
  nodeOffset = 12,
  searchInput,
  displayMode = "LIVE",
  showOrders = false,
  showRootNodeIds = false,
}: ProjectTreeProps) => {
  return (
    <ProjectTreeContext.Provider
      value={{
        id: tree.id,
        name: tree.name,
        order: tree.order ?? 0,
        iconPath: tree.iconPath,
        treePaddingLeft,
        treePaddingRight,
        nodeOffset,
        allFoldersAreExpanded: checkIfAllFoldersAreExpanded(tree),
        allFoldersAreCollapsed: checkIfAllFoldersAreCollapsed(tree),
        searchInput: searchInput ?? "",
        displayMode,
        showOrders,
        showRootNodeIds,
      }}
    >
      <TreeRootNode node={tree} />
    </ProjectTreeContext.Provider>
  );
};

export default ProjectTree;
