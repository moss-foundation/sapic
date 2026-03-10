import { useContext, useRef } from "react";

import { useCurrentWorkspace } from "@/hooks";
import { Tree } from "@/lib/ui/Tree";
import { useGetResourcesListItemState } from "@/workbench/adapters/tanstackQuery/resourcesListItemState/useGetResourcesListItemState";

import { ProjectTreeContext } from "../ProjectTreeContext";
import { IResourcesTree } from "../types";
import { useDropTargetResourcesList } from "./dnd/hooks/useDropTargetResourcesList";
import ResourceNodeAddForm from "./forms/ResourceNodeAddForm";
import { useRootResourceNodeAddForm } from "./hooks/useRootResourceNodeAddForm";
import { ResourcesTreeChildren } from "./ResourcesTreeChildren";
import { ResourcesTreeHeader } from "./ResourcesTreeHeader";

interface ResourcesTreeProps {
  tree: IResourcesTree;
}

export const ResourcesTree = ({ tree }: ResourcesTreeProps) => {
  const { currentWorkspaceId } = useCurrentWorkspace();
  const { id, treePaddingLeft, treePaddingRight } = useContext(ProjectTreeContext);

  const projectResourcesHeaderRef = useRef<HTMLHeadingElement>(null);
  const listHeaderOffset = treePaddingLeft * 2;

  const { data: expanded = false } = useGetResourcesListItemState(id, currentWorkspaceId);

  const { instruction } = useDropTargetResourcesList({
    ref: projectResourcesHeaderRef,
    rootResourcesNodes: tree.childNodes,
  });

  const {
    isAddingFileNode,
    isAddingFolderNode,
    setIsAddingFileNode,
    setIsAddingFolderNode,
    handleAddFormSubmit,
    handleAddFormCancel,
  } = useRootResourceNodeAddForm({ tree });

  const isAddingRootNode = isAddingFileNode || isAddingFolderNode;
  const shouldRenderChildren = expanded || isAddingRootNode;

  return (
    <Tree.List combineInstruction={instruction}>
      <ResourcesTreeHeader
        expanded={expanded}
        offsetLeft={listHeaderOffset}
        offsetRight={treePaddingRight}
        ref={projectResourcesHeaderRef}
        setIsAddingFileNode={() => setIsAddingFileNode(true)}
        setIsAddingFolderNode={() => setIsAddingFolderNode(true)}
      />

      {shouldRenderChildren && (
        <ResourcesTreeChildren rootResourcesNodes={tree.childNodes} parentNode={tree} depth={1} />
      )}

      {isAddingRootNode && (
        <ResourceNodeAddForm
          depth={0}
          isAddingFolderNode={isAddingFolderNode}
          handleAddFormSubmit={handleAddFormSubmit}
          handleAddFormCancel={handleAddFormCancel}
          restrictedNames={[]}
        />
      )}
    </Tree.List>
  );
};
