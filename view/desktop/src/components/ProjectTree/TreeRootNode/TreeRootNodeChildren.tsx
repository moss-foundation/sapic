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
}

export const TreeRootNodeChildren = ({
  node,
  isAddingRootFileNode,
  isAddingRootFolderNode,
  handleAddFormRootSubmit,
  handleAddFormRootCancel,
}: TreeRootNodeChildrenProps) => {
  const shouldRenderAddRootForm = isAddingRootFileNode || isAddingRootFolderNode;
  const restrictedNames = getChildrenNames(node);

  const sortedChildNodes = sortObjectsByOrder(node.childNodes);

  return (
    <Tree.RootNodeChildren>
      {sortedChildNodes.map((childNode, index) => {
        return (
          <TreeNode
            parentNode={node}
            key={childNode.id}
            node={childNode}
            depth={1}
            isLastChild={index === sortedChildNodes.length - 1}
          />
        );
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
