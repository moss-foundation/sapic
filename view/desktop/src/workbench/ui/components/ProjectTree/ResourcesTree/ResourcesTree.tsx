import { useContext, useRef } from "react";

import { useCurrentWorkspace } from "@/hooks";
import { Tree } from "@/lib/ui/Tree";
import { useGetResourcesListItemState } from "@/workbench/adapters/tanstackQuery/resourcesListItemState/useGetResourcesListItemState";

import { NODE_OFFSET, TREE_HEADER_PADDING_RIGHT } from "../constants";
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

const listHeaderOffset = NODE_OFFSET * 3;

export const ResourcesTree = ({ tree }: ResourcesTreeProps) => {
  const { currentWorkspaceId } = useCurrentWorkspace();
  const { id } = useContext(ProjectTreeContext);

  const projectResourcesHeaderRef = useRef<HTMLHeadingElement>(null);

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

  const isAddingToListRoot = isAddingFileNode || isAddingFolderNode;
  const shouldRenderChildren = expanded || isAddingToListRoot;

  return (
    <Tree.List combineInstruction={instruction}>
      <ResourcesTreeHeader
        expanded={expanded}
        offsetLeft={listHeaderOffset}
        offsetRight={TREE_HEADER_PADDING_RIGHT}
        ref={projectResourcesHeaderRef}
        setIsAddingFileNode={() => setIsAddingFileNode(true)}
        setIsAddingFolderNode={() => setIsAddingFolderNode(true)}
      />

      {shouldRenderChildren && (
        <ResourcesTreeChildren rootResourcesNodes={tree.childNodes} parentNode={tree} depth={4} />
      )}

      {isAddingToListRoot && (
        <ResourceNodeAddForm
          depth={1}
          isAddingFolderNode={isAddingFolderNode}
          handleAddFormSubmit={handleAddFormSubmit}
          handleAddFormCancel={handleAddFormCancel}
          restrictedNames={[]}
        />
      )}
    </Tree.List>
  );
};
