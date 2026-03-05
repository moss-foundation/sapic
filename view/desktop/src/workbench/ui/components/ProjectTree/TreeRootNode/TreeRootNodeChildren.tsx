import { Tree } from "@/lib/ui/Tree";
import { sortObjectsByOrder } from "@/utils/sortObjectsByOrder";

import TreeNode from "../TreeNode/TreeNode";
import { ProjectTreeRootNode } from "../types";
import { getChildrenNames } from "../utils";
import { TreeRootNodeAddForm } from "./TreeRootNodeAddForm";

interface TreeRootNodeChildrenProps {
  node: ProjectTreeRootNode;
  isAddingRootFileNode: boolean;
  isAddingRootFolderNode: boolean;
  handleAddFormRootSubmit: (name: string) => void;
  handleAddFormRootCancel: () => void;
  offsetLeft?: number;
  depth?: number;
}

export const TreeRootNodeChildren = ({
  node,
  isAddingRootFileNode,
  isAddingRootFolderNode,
  handleAddFormRootSubmit,
  handleAddFormRootCancel,
  offsetLeft,
  depth,
}: TreeRootNodeChildrenProps) => {
  const shouldRenderAddRootForm = isAddingRootFileNode || isAddingRootFolderNode;
  const restrictedNames = getChildrenNames(node);
  const sortedChildNodes = sortObjectsByOrder(node.childNodes);

  return (
    <Tree.RootNodeChildren offset={offsetLeft} depth={depth} treeOffset={6}>
      {sortedChildNodes.map((childNode) => {
        return <TreeNode key={childNode.id} node={childNode} depth={3} />;
      })}

      {shouldRenderAddRootForm && (
        <TreeRootNodeAddForm
          handleAddFormRootSubmit={handleAddFormRootSubmit}
          handleAddFormRootCancel={handleAddFormRootCancel}
          restrictedNames={restrictedNames}
        />
      )}
    </Tree.RootNodeChildren>
  );
};
