import { ProjectTreeContext, TreeContextBridge } from "./ProjectTreeContext.tsx";
import { TreeRootNode } from "./TreeRootNode/TreeRootNode.tsx";
import { ProjectTreeProps } from "./types.ts";
import { checkIfAllFoldersAreCollapsed, checkIfAllFoldersAreExpanded } from "./utils/TreeRoot.ts";

export const ProjectTree = ({
  tree,
  treePaddingLeft = 12,
  treePaddingRight = 8,
  nodeOffset = 12,
  searchInput,
  displayMode = "LIVE",
  showOrders = false,
  showRootNodeIds = false,
}: ProjectTreeProps) => {
  console.log("tree", tree.id);
  return (
    <div>
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
        <TreeContextBridge>
          <TreeRootNode node={tree} />
        </TreeContextBridge>
      </ProjectTreeContext.Provider>
    </div>
  );
};

export default ProjectTree;
