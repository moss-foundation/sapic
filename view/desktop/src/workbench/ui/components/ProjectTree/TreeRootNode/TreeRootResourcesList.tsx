import { useContext, useState } from "react";

import { Icon } from "@/lib/ui";
import { Tree } from "@/lib/ui/Tree";
import { cn } from "@/utils";

import { ProjectTreeContext } from "../ProjectTreeContext";
import { ProjectTreeRootNode } from "../types";
import { calculateShouldRenderRootChildNodes } from "../utils/TreeRoot";
import { TreeRootNodeChildren } from "./TreeRootNodeChildren";

interface TreeRootResourcesListProps {
  tree: ProjectTreeRootNode;
  isAddingRootFileNode: boolean;
  isAddingRootFolderNode: boolean;
  handleRootAddFormSubmit: (name: string) => void;
  handleRootAddFormCancel: () => void;
}

export const TreeRootResourcesList = ({
  tree,
  isAddingRootFileNode,
  isAddingRootFolderNode,
  handleRootAddFormSubmit,
  handleRootAddFormCancel,
}: TreeRootResourcesListProps) => {
  const { treePaddingLeft } = useContext(ProjectTreeContext);

  //FIXME this a temporary solution to expand the resources list
  const [expanded, setExpanded] = useState(false);
  const handleExpand = () => {
    setExpanded(!expanded);
  };
  // const shouldRenderResourcesList = calculateShouldRenderRootChildNodes(tree, false, false);

  //if (!shouldRenderResourcesList) return null;

  return (
    <Tree.Node>
      <Tree.NodeControls
        className="hover:background-(--moss-list-background-hover) flex cursor-pointer items-center gap-1 py-[5px]"
        style={{ paddingLeft: treePaddingLeft }}
        onClick={handleExpand}
      >
        <Tree.RootNodeTriggers>
          <Icon icon="ChevronRight" className={cn(expanded && "rotate-90")} />
          <div className="flex items-center gap-1">
            <Tree.RootNodeLabel label="Resources" />
            {/* <Tree.NodeDirCount count={123} /> */}
          </div>
        </Tree.RootNodeTriggers>
      </Tree.NodeControls>

      {expanded && (
        <TreeRootNodeChildren
          node={tree}
          isAddingRootFileNode={isAddingRootFileNode}
          isAddingRootFolderNode={isAddingRootFolderNode}
          handleAddFormRootSubmit={handleRootAddFormSubmit}
          handleAddFormRootCancel={handleRootAddFormCancel}
        />
      )}
    </Tree.Node>
  );
};
