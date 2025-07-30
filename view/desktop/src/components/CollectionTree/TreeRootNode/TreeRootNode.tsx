import { useContext, useRef } from "react";

import { useStreamedCollections } from "@/hooks";
import { cn } from "@/utils";

import { DropIndicatorWithInstruction } from "../DropIndicatorWithInstruction";
import { useDraggableRootNode } from "../hooks/useDraggableRootNode";
import { useRootNodeAddForm } from "../hooks/useRootNodeAddForm";
import { useRootNodeRenamingForm } from "../hooks/useRootNodeRenamingForm";
import { TreeContext } from "../Tree";
import { TreeRootNodeProps } from "../types";
import { calculateShouldRenderRootChildNodes } from "../utils";
import { TreeRootNodeActions } from "./TreeRootNodeActions";
import { TreeRootNodeButton } from "./TreeRootNodeButton";
import { TreeRootNodeChildren } from "./TreeRootNodeChildren";
import { TreeRootNodeRenameForm } from "./TreeRootNodeRenameForm";

export const TreeRootNode = ({ node }: TreeRootNodeProps) => {
  const { searchInput, treePaddingLeft, treePaddingRight } = useContext(TreeContext);

  const { data: streamedCollections } = useStreamedCollections();

  const draggableRootRef = useRef<HTMLDivElement>(null);
  const dropTargetRootRef = useRef<HTMLDivElement>(null);

  const {
    isAddingRootNodeFile,
    isAddingRootNodeFolder,
    setIsAddingRootNodeFile,
    setIsAddingRootNodeFolder,
    handleRootAddFormCancel,
    handleRootAddFormSubmit,
  } = useRootNodeAddForm(node);

  const {
    isRenamingRootNode,
    setIsRenamingRootNode,
    handleRenamingRootNodeFormSubmit,
    handleRenamingRootNodeFormCancel,
  } = useRootNodeRenamingForm(node);

  const { instruction, isDragging, canDrop } = useDraggableRootNode(draggableRootRef, node, isRenamingRootNode);

  const shouldRenderRootChildNodes = calculateShouldRenderRootChildNodes(
    node,
    isDragging,
    isAddingRootNodeFile,
    isRenamingRootNode
  );

  const restrictedNames = streamedCollections?.map((collection) => collection.name) ?? [];

  return (
    <div
      ref={dropTargetRootRef}
      className={cn("group relative w-full", {
        "hidden": isDragging,
      })}
    >
      {instruction && <DropIndicatorWithInstruction instruction={instruction} gap={-1} canDrop={canDrop} />}
      <div
        ref={draggableRootRef}
        className="group/TreeRootHeader relative flex w-full min-w-0 items-center justify-between py-0.5"
        style={{
          paddingLeft: treePaddingLeft,
          paddingRight: treePaddingRight,
        }}
      >
        <span
          style={{
            width: `calc(100% - ${treePaddingLeft}px - ${treePaddingRight}px)`,
            inset: `0 ${treePaddingLeft}px 0 ${treePaddingRight}px`,
          }}
          className={cn(
            "group-hover/TreeRootHeader:background-(--moss-secondary-background-hover) absolute h-full rounded-sm",
            {
              "group-hover/TreeRootHeader:background-transparent": isRenamingRootNode,
            }
          )}
        />

        {isRenamingRootNode ? (
          <TreeRootNodeRenameForm
            node={node}
            restrictedNames={restrictedNames}
            handleRenamingFormSubmit={handleRenamingRootNodeFormSubmit}
            handleRenamingFormCancel={handleRenamingRootNodeFormCancel}
          />
        ) : (
          <TreeRootNodeButton
            node={node}
            searchInput={searchInput}
            shouldRenderChildNodes={shouldRenderRootChildNodes}
          />
        )}

        <TreeRootNodeActions
          node={node}
          searchInput={searchInput}
          isRenamingRootNode={isRenamingRootNode}
          setIsAddingRootFileNode={setIsAddingRootNodeFile}
          setIsAddingRootFolderNode={setIsAddingRootNodeFolder}
          setIsRenamingRootNode={setIsRenamingRootNode}
        />
      </div>
      {shouldRenderRootChildNodes && (
        <TreeRootNodeChildren
          node={node}
          isAddingRootFileNode={isAddingRootNodeFile}
          isAddingRootFolderNode={isAddingRootNodeFolder}
          handleAddFormRootSubmit={handleRootAddFormSubmit}
          handleAddFormRootCancel={handleRootAddFormCancel}
        />
      )}
    </div>
  );
};
