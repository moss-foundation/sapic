import { ProjectTreeContext } from "./ProjectTreeContext.tsx";
import { TreeRootNode } from "./TreeRootNode/TreeRootNode.tsx";
import { ProjectTreeProps } from "./types.ts";
import { checkIfAllFoldersAreCollapsed, checkIfAllFoldersAreExpanded } from "./utils/TreeRoot.ts";

export const ProjectTree = ({
  tree,
  searchInput,
  displayMode = "LIVE",
  showOrders = false,
  showRootNodeIds = false,
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
          showRootNodeIds,
        }}
      >
        <TreeRootNode tree={tree} />
      </ProjectTreeContext.Provider>
    </div>
  );
};

export default ProjectTree;
