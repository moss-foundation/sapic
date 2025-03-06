import { useContext, useMemo, useRef, useState } from "react";
import { createPortal } from "react-dom";

import { cn } from "@/utils";

import { ContextMenu, Icon, TreeContext } from "..";
import { useDraggableNode } from "./hooks/useDraggableNode";
import { useDropTargetNode } from "./hooks/useDropTargetNode";
import { useNodeAddForm } from "./hooks/useNodeAddForm";
import { useNodeRenamingForm } from "./hooks/useNodeRenamingForm";
import { NodeAddForm } from "./NodeAddForm";
import NodeLabel from "./NodeLabel";
import { NodeRenamingForm } from "./NodeRenamingForm";
import { TreeNodeComponentProps } from "./types";
import { collapseAllNodes, expandAllNodes, hasDescendantWithSearchInput } from "./utils";

export const TreeNode = ({ node, onNodeUpdate, depth, parentNode }: TreeNodeComponentProps) => {
  const { treeId, horizontalPadding, nodeOffset, allFoldersAreCollapsed, allFoldersAreExpanded, searchInput } =
    useContext(TreeContext);

  const paddingLeft = useMemo(
    () => `${depth * nodeOffset + horizontalPadding}px`,
    [depth, nodeOffset, horizontalPadding]
  );
  const paddingRight = useMemo(() => `${horizontalPadding}px`, [horizontalPadding]);

  const [preview, setPreview] = useState<HTMLElement | null>(null);

  const draggableRef = useRef<HTMLButtonElement>(null);
  const dropTargetFolderRef = useRef<HTMLUListElement>(null);
  const dropTargetListRef = useRef<HTMLLIElement>(null);

  const { isRenamingNode, setIsRenamingNode, handleRenamingFormSubmit, handleRenamingFormCancel } = useNodeRenamingForm(
    node,
    onNodeUpdate
  );

  const {
    isAddingFileNode,
    isAddingFolderNode,
    setIsAddingFileNode,
    setIsAddingFolderNode,
    handleAddFormSubmit,
    handleAddFormCancel,
  } = useNodeAddForm(node, onNodeUpdate);

  useDraggableNode(draggableRef, node, treeId, isRenamingNode, setPreview);
  useDropTargetNode(node, treeId, dropTargetListRef, dropTargetFolderRef);

  const shouldRenderChildNodes =
    !!searchInput || isAddingFileNode || isAddingFolderNode || (!searchInput && node.isFolder && node.isExpanded);

  const filteredChildNodes = searchInput
    ? node.childNodes.filter((childNode) => hasDescendantWithSearchInput(childNode, searchInput))
    : node.childNodes;

  const handleFolderClick = () => {
    if (!node.isFolder || searchInput) return;

    onNodeUpdate({
      ...node,
      isExpanded: !node.isExpanded,
    });
  };

  const handleExpandAll = () => {
    const newNode = expandAllNodes(node);
    onNodeUpdate({
      ...node,
      childNodes: newNode.childNodes,
    });
  };

  const handleCollapseAll = () => {
    const newNode = collapseAllNodes(node);
    onNodeUpdate({
      ...node,
      childNodes: newNode.childNodes,
    });
  };

  if (node.id === "root") {
    return (
      <div className="flex flex-col h-full">
        <div className="flex w-full min-w-0 py-1 pr-2 items-center justify-between gap-1 focus-within:bg-[#ebecf0] dark:focus-within:bg-[#434343] ">
          <button className="flex gap-1 items-center grow cursor-pointer" onClick={handleFolderClick}>
            <Icon
              icon="TreeChevronRightIcon"
              className={cn("text-[#717171]", {
                "rotate-90": shouldRenderChildNodes,
              })}
            />

            <NodeLabel label={node.id} searchInput={searchInput} />
          </button>

          <div className="flex gap-1 items-center">
            {node.isExpanded && !searchInput && (
              <>
                {!allFoldersAreExpanded && (
                  <button
                    className="size-[22px] text-[#717171] hover:text-[#6C707E] hover:bg-[#EBECF0] hover:dark:bg-black/30  flex items-center justify-center rounded-[3px] cursor-pointer"
                    onClick={handleExpandAll}
                  >
                    <Icon icon="TreeExpandAllIcon" />
                  </button>
                )}

                {!allFoldersAreCollapsed && (
                  <button
                    className="size-[22px] text-[#717171] hover:text-[#6C707E] hover:bg-[#EBECF0] hover:dark:bg-black/30  flex items-center justify-center rounded-[3px] cursor-pointer"
                    onClick={handleCollapseAll}
                  >
                    <Icon icon="TreeCollapseAllIcon" />
                  </button>
                )}
              </>
            )}
            <button className="size-[22px] text-[#717171] hover:text-[#6C707E] hover:bg-[#EBECF0] hover:dark:bg-black/30  flex items-center justify-center rounded-[3px] cursor-pointer">
              <Icon icon="TreeDetailIcon" />
            </button>
          </div>
        </div>

        {shouldRenderChildNodes && (
          <ul ref={dropTargetFolderRef} className="grow">
            {filteredChildNodes.map((childNode) => (
              <TreeNode
                parentNode={node}
                onNodeUpdate={onNodeUpdate}
                key={childNode.uniqueId}
                node={childNode}
                depth={0}
              />
            ))}
          </ul>
        )}
      </div>
    );
  }

  return (
    <li ref={dropTargetListRef}>
      {isRenamingNode ? (
        <div className="flex w-full min-w-0 items-center gap-1" style={{ paddingLeft }}>
          <Icon icon={node.isFolder ? "TreeFolderIcon" : "TreeFileIcon"} className="ml-auto" />
          <NodeRenamingForm
            onSubmit={(e) => {
              console.log(4.5);
              handleRenamingFormSubmit(e);
            }}
            onCancel={handleRenamingFormCancel}
            restrictedNames={parentNode.childNodes.map((childNode) => childNode.id)}
            currentName={node.id}
          />
        </div>
      ) : (
        <ContextMenu.Root modal={false}>
          <ContextMenu.Trigger asChild>
            <button
              ref={draggableRef}
              style={{ paddingLeft, paddingRight }}
              onClick={node.isFolder ? handleFolderClick : undefined}
              className="flex gap-1 w-full min-w-0 grow items-center cursor-pointer focus-within:outline-none focus-within:bg-[#ebecf0] dark:focus-within:bg-[#747474] relative hover:bg-[#ebecf0] dark:hover:bg-[#434343]"
            >
              <Icon icon={node.isFolder ? "TreeFolderIcon" : "TreeFileIcon"} />

              <NodeLabel label={node.id} searchInput={searchInput} />

              <span className="DragHandle h-full min-h-4 grow" />

              <Icon
                icon="TreeChevronRightIcon"
                className={cn("ml-auto text-[#717171]", {
                  "rotate-90": shouldRenderChildNodes,
                  "opacity-0": !node.isFolder,
                })}
              />

              {preview &&
                createPortal(
                  <ul className="bg-[#ebecf0] dark:bg-[#434343]">
                    <TreeNode
                      parentNode={{
                        uniqueId: "-",
                        childNodes: [],
                        type: "",
                        order: 0,
                        isFolder: false,
                        isExpanded: false,
                        id: "-",
                      }}
                      node={{ ...node, childNodes: [] }}
                      onNodeUpdate={() => {}}
                      depth={0}
                    />
                  </ul>,
                  preview
                )}
            </button>
          </ContextMenu.Trigger>

          <ContextMenu.Portal>
            <ContextMenu.Content className="text-white">
              {node.isFolder && <ContextMenu.Item label="Add File" onClick={() => setIsAddingFileNode(true)} />}
              {node.isFolder && <ContextMenu.Item label="Add Folder" onClick={() => setIsAddingFolderNode(true)} />}
              <ContextMenu.Item label="Edit" onClick={() => setIsRenamingNode(true)} />
              <ContextMenu.Item label="Item" />
            </ContextMenu.Content>
          </ContextMenu.Portal>
        </ContextMenu.Root>
      )}

      {(isAddingFileNode || isAddingFolderNode) && (
        <div
          style={{ paddingLeft: `${(depth + 1) * nodeOffset + horizontalPadding}px` }}
          className="flex w-full min-w-0 items-center gap-1"
        >
          <Icon icon={isAddingFolderNode ? "TreeFolderIcon" : "TreeFileIcon"} className="ml-auto" />
          <NodeAddForm
            isFolder={isAddingFolderNode}
            restrictedNames={node.childNodes.map((childNode) => childNode.id)}
            onSubmit={handleAddFormSubmit}
            onCancel={handleAddFormCancel}
          />
        </div>
      )}

      {shouldRenderChildNodes && (
        <ul ref={dropTargetFolderRef} className="h-full">
          {filteredChildNodes.map((childNode) => (
            <TreeNode
              parentNode={node}
              onNodeUpdate={onNodeUpdate}
              key={childNode.uniqueId}
              node={childNode}
              depth={depth + 1}
            />
          ))}
        </ul>
      )}
    </li>
  );
};

export default TreeNode;
