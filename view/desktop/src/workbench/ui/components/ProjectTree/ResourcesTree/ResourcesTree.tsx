import { useContext, useRef } from "react";

import { useCurrentWorkspace } from "@/hooks";
import { Tree } from "@/lib/ui/Tree";
import { useGetResourcesListItemState } from "@/workbench/adapters/tanstackQuery/resourcesListItemState/useGetResourcesListItemState";

import { NODE_OFFSET, TREE_HEADER_PADDING_RIGHT } from "../constants";
import { ProjectTreeContext } from "../ProjectTreeContext";
import { ResourcesTreeRoot } from "../TreeRoot/types";
import { useDropTargetResourcesList } from "./dnd/hooks/useDropTargetResourcesList";
import ResourceNodeAddForm from "./forms/ResourceNodeAddForm";
import { useRootResourceNodeAddForm } from "./hooks/useRootResourceNodeAddForm";
import { ResourcesTreeChildren } from "./ResourcesTreeChildren";
import { ResourcesTreeHeader } from "./ResourcesTreeHeader";
import { countNumberOfAllNestedItems } from "./utils/countNumberOfAllNestedItems";

interface ResourcesTreeProps {
  tree: ResourcesTreeRoot;
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
  const totalItemsCount = tree.childNodes.reduce((acc, child) => {
    const selfCount = child.kind === "Item" ? 1 : 0;
    return acc + selfCount + countNumberOfAllNestedItems(child);
  }, 0);

  return (
    <Tree.List combineInstruction={instruction}>
      <ResourcesTreeHeader
        expanded={expanded}
        offsetLeft={listHeaderOffset}
        offsetRight={TREE_HEADER_PADDING_RIGHT}
        ref={projectResourcesHeaderRef}
        setIsAddingFileNode={() => setIsAddingFileNode(true)}
        setIsAddingFolderNode={() => setIsAddingFolderNode(true)}
        totalItemsCount={totalItemsCount}
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
