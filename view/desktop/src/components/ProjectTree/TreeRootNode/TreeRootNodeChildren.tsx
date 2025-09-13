import { useContext } from "react";

import { Tree } from "@/lib/ui/Tree";
import { sortObjectsByOrder } from "@/utils/sortObjectsByOrder";

import { EntryIcon } from "../../EntryIcon";
import { ProjectTreeContext } from "../ProjectTreeContext";
import TreeNode from "../TreeNode/TreeNode";
import { ProjectTreeRootNode } from "../types";
import { getChildrenNames } from "../utils";

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
  const { nodeOffset, displayMode } = useContext(ProjectTreeContext);

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
            isRootNode={displayMode === "DESIGN_FIRST"}
          />
        );
      })}

      {shouldRenderAddRootForm && (
        <div className="flex w-full min-w-0 items-center gap-1 py-0.5" style={{ paddingLeft: nodeOffset * 1 }}>
          <EntryIcon />
          <Tree.NodeAddForm
            onSubmit={handleAddFormRootSubmit}
            onCancel={handleAddFormRootCancel}
            restrictedNames={restrictedNames}
          />
        </div>
      )}
    </Tree.RootNodeChildren>
  );
};
