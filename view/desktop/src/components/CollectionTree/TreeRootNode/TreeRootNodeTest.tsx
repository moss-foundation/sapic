import { useContext, useRef } from "react";

import ActionButton from "@/components/ActionButton";
import { Tree } from "@/components/Tree";
import { useStreamCollections, useUpdateCollection } from "@/hooks";
import { useTabbedPaneStore } from "@/store/tabbedPane";

import { CollectionTreeContextTest } from "../CollectionTreeContextTest";
import { TreeRootNodeProps } from "../types";
import { calculateShouldRenderRootChildNodes } from "../utils";
import { useDraggableRootNode } from "./hooks/useDraggableRootNode";
import { useRootNodeAddForm } from "./hooks/useRootNodeAddForm";
import { useRootNodeRenamingForm } from "./hooks/useRootNodeRenamingForm";

export const TreeRootNodeTest = ({ node }: TreeRootNodeProps) => {
  const { id } = useContext(CollectionTreeContextTest);

  const draggableRootRef = useRef<HTMLDivElement>(null);
  const dropTargetRootRef = useRef<HTMLDivElement>(null);

  const { data: streamedCollections } = useStreamCollections();
  const { activePanelId } = useTabbedPaneStore();

  const {
    isAddingRootFileNode,
    isAddingRootFolderNode,
    setIsAddingRootFileNode,
    setIsAddingRootFolderNode,
    handleRootAddFormCancel,
    handleRootAddFormSubmit,
  } = useRootNodeAddForm(node);

  const {
    isRenamingRootNode,
    setIsRenamingRootNode,
    handleRenamingRootNodeFormSubmit,
    handleRenamingRootNodeFormCancel,
  } = useRootNodeRenamingForm(node);

  const { isDragging, isChildDropBlocked, instruction } = useDraggableRootNode({
    dirRef: dropTargetRootRef,
    triggerRef: draggableRootRef,
    node,
    isRenamingNode: isRenamingRootNode,
  });

  const shouldRenderRootChildNodes = calculateShouldRenderRootChildNodes(
    node,
    isDragging,
    isAddingRootFileNode,
    isRenamingRootNode
  );

  const restrictedNames = streamedCollections?.map((collection) => collection.name) ?? [];

  const { mutateAsync: updateCollection } = useUpdateCollection();

  const handleIconClick = async (e: React.MouseEvent<HTMLButtonElement>) => {
    e.stopPropagation();

    await updateCollection({
      id,
      expanded: !node.expanded,
    });
  };

  return (
    //Root Node
    <Tree.RootNode isChildDropBlocked={isChildDropBlocked} instruction={instruction}>
      <Tree.RootNodeHeader draggableHeaderRef={draggableRootRef} isActive={true}>
        {isRenamingRootNode ? (
          <Tree.RootNodeRenameForm
            name={node.name}
            shouldRenderChildNodes={shouldRenderRootChildNodes}
            restrictedNames={restrictedNames}
            handleRenamingFormSubmit={handleRenamingRootNodeFormSubmit}
            handleRenamingFormCancel={handleRenamingRootNodeFormCancel}
          />
        ) : (
          <>
            <Tree.RootNodeTriggers>
              <Tree.RootNodeIcon
                handleIconClick={handleIconClick}
                areChildrenShown={shouldRenderRootChildNodes}
                iconPath=""
              />
              <Tree.RootNodeOrder order={1} />
              <Tree.RootNodeLabel label="Node Label" />
            </Tree.RootNodeTriggers>
            <Tree.RootNodeActions>
              <Tree.ActionLabel>main</Tree.ActionLabel>

              <Tree.HoverActions>
                <ActionButton icon="Add" onClick={() => setIsAddingRootNodeFile(true)} />
                <ActionButton icon="Refresh" />
              </Tree.HoverActions>

              <Tree.PersistentActions>
                <ActionButton icon="MoreHorizontal" onClick={() => setIsRenamingRootNode(true)} />
              </Tree.PersistentActions>
            </Tree.RootNodeActions>
          </>
        )}
      </Tree.RootNodeHeader>

      <Tree.RootNodeChildren shouldRenderChildNodes={shouldRenderRootChildNodes} className="pl-5">
        <div>1</div>
        <div>2</div>
        <div>3</div>
      </Tree.RootNodeChildren>
    </Tree.RootNode>
  );
};
