import { useContext, useRef } from "react";

import { useCurrentWorkspace } from "@/hooks";
import { Tree } from "@/lib/ui/Tree";
import { useGetResourcesListItemState } from "@/workbench/adapters/tanstackQuery/resourcesListItemState/useGetResourcesListItemState";

import { ProjectTreeContext } from "../ProjectTreeContext";
import { IResourcesTree } from "../types";
import { useDropTargetResourcesList } from "./dnd/hooks/useDropTargetResourcesList";
import ResourceNodeAddForm from "./forms/ResourceNodeAddForm";
import { ResourcesTreeChildren } from "./ResourcesTreeChildren";
import { ResourcesTreeHeader } from "./ResourcesTreeHeader";

interface ResourcesTreeProps {
  tree: IResourcesTree;
  isAddingRootFileNode: boolean;
  isAddingRootFolderNode: boolean;
  handleRootAddFormSubmit: (name: string) => void;
  handleRootAddFormCancel: () => void;
}

export const ResourcesTree = ({
  tree,
  isAddingRootFileNode,
  isAddingRootFolderNode,
  handleRootAddFormSubmit,
  handleRootAddFormCancel,
}: ResourcesTreeProps) => {
  const { currentWorkspaceId } = useCurrentWorkspace();
  const { id, treePaddingLeft } = useContext(ProjectTreeContext);

  const projectResourcesHeaderRef = useRef<HTMLHeadingElement>(null);
  const listHeaderOffset = treePaddingLeft * 2;

  const { data: expanded = false } = useGetResourcesListItemState(id, currentWorkspaceId);

  const { instruction } = useDropTargetResourcesList({
    ref: projectResourcesHeaderRef,
    rootResourcesNodes: tree.childNodes,
  });

  const shouldRenderChildren = expanded || isAddingRootFileNode;
  const isAddingRootNode = isAddingRootFileNode || isAddingRootFolderNode;

  return (
    <Tree.List combineInstruction={instruction}>
      <ResourcesTreeHeader expanded={expanded} offsetLeft={listHeaderOffset} ref={projectResourcesHeaderRef} />

      {shouldRenderChildren && (
        <ResourcesTreeChildren rootResourcesNodes={tree.childNodes} parentNode={tree} depth={1} />
      )}
      {isAddingRootNode && (
        <ResourceNodeAddForm
          depth={1}
          isAddingFolderNode={isAddingRootFolderNode}
          handleAddFormSubmit={handleRootAddFormSubmit}
          handleAddFormCancel={handleRootAddFormCancel}
          restrictedNames={[]}
        />
      )}
    </Tree.List>
  );
};
