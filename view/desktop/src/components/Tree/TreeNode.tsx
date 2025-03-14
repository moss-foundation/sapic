import { useContext, useEffect, useMemo, useRef, useState } from "react";
import { createPortal } from "react-dom";

import { cn } from "@/utils";

import { ContextMenu, DropdownMenu, DropIndicator, Icon, Scrollbar, TreeContext } from "..";
import { useDraggableNode } from "./hooks/useDraggableNode";
import { useDraggableRootNode } from "./hooks/useDraggableRootNode";
import { useDropTargetNode } from "./hooks/useDropTargetNode";
import { useNodeAddForm } from "./hooks/useNodeAddForm";
import { useNodeRenamingForm } from "./hooks/useNodeRenamingForm";
import { NodeAddForm } from "./NodeAddForm";
import NodeLabel from "./NodeLabel";
import { NodeRenamingForm } from "./NodeRenamingForm";
import { TreeNodeComponentProps } from "./types";
import { collapseAllNodes, expandAllNodes, hasDescendantWithSearchInput } from "./utils";

export const TreeNode = ({ node, onNodeUpdate, depth, parentNode }: TreeNodeComponentProps) => {
  const { treeId, nodeOffset, paddingLeft, paddingRight, allFoldersAreCollapsed, allFoldersAreExpanded, searchInput } =
    useContext(TreeContext);

  const nodePaddingLeft = useMemo(() => depth * nodeOffset + paddingLeft + 4, [depth, nodeOffset, paddingLeft]);

  const nodeStyle = useMemo(() => "flex w-full min-w-0 items-center gap-1 py-0.5", []);

  const [preview, setPreview] = useState<HTMLElement | null>(null);

  const draggableRootRef = useRef<HTMLDivElement>(null);
  const draggableNodeRef = useRef<HTMLButtonElement>(null);
  const dropTargetFolderRef = useRef<HTMLDivElement>(null);
  const dropTargetListRef = useRef<HTMLLIElement>(null);

  const {
    isAddingFileNode,
    isAddingFolderNode,
    setIsAddingFileNode,
    setIsAddingFolderNode,
    handleAddFormSubmit,
    handleAddFormCancel,
  } = useNodeAddForm(node, onNodeUpdate);

  const {
    isAddingFileNode: isAddingRootFileNode,
    isAddingFolderNode: isAddingRootFolderNode,
    setIsAddingFileNode: setIsAddingRootFileNode,
    setIsAddingFolderNode: setIsAddingRootFolderNode,
    handleAddFormSubmit: handleAddFormRootSubmit,
    handleAddFormCancel: handleAddFormRootCancel,
  } = useNodeAddForm(node, onNodeUpdate);

  const { isRenamingNode, setIsRenamingNode, handleRenamingFormSubmit, handleRenamingFormCancel } = useNodeRenamingForm(
    node,
    onNodeUpdate
  );

  const {
    isRenamingNode: isRenamingRootNode,
    setIsRenamingNode: setIsRenamingRootNode,
    handleRenamingFormSubmit: handleRenamingRootFormSubmit,
    handleRenamingFormCancel: handleRenamingRootFormCancel,
  } = useNodeRenamingForm(node, onNodeUpdate);

  const { closestEdge, isDragging: isRootDragging } = useDraggableRootNode(
    draggableRootRef,
    node,
    treeId,
    isRenamingRootNode
  );
  const { isDragging: isNodeDragging } = useDraggableNode(draggableNodeRef, node, treeId, isRenamingNode, setPreview);
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

  useEffect(() => {
    const handleNewCollectionWasCreated = (event: Event) => {
      const customEvent = event as CustomEvent<{ treeId: string }>;
      if (node.isRoot && treeId === customEvent.detail.treeId) {
        setIsRenamingRootNode(true);
      }
    };

    window.addEventListener("newCollectionWasCreated", handleNewCollectionWasCreated);

    return () => {
      window.removeEventListener("newCollectionWasCreated", handleNewCollectionWasCreated as EventListener);
    };
  }, [node.isRoot, setIsRenamingRootNode, treeId]);

  if (node.isRoot) {
    return (
      <div ref={dropTargetFolderRef} className={cn("group relative w-full border-b border-b-(--moss-tree-border)")}>
        <div
          ref={draggableRootRef}
          className="focus-within:background-(--moss-treeNode-bg) flex w-full min-w-0 items-center justify-between gap-1 py-[7px] pr-2"
          style={{ paddingLeft, paddingRight }}
        >
          {isRenamingRootNode ? (
            <div className="flex grow cursor-pointer items-center gap-1">
              <Icon
                icon="TreeChevronRightIcon"
                className={cn("text-[#717171]", {
                  "rotate-90": shouldRenderChildNodes,
                })}
              />
              <NodeRenamingForm
                onSubmit={handleRenamingRootFormSubmit}
                onCancel={handleRenamingRootFormCancel}
                currentName={node.id}
              />
            </div>
          ) : (
            <button className="flex grow cursor-pointer items-center gap-1 overflow-hidden" onClick={handleFolderClick}>
              <Icon
                icon="TreeChevronRightIcon"
                className={cn("text-[#717171]", {
                  "rotate-90": shouldRenderChildNodes,
                })}
              />

              <NodeLabel label={node.id} searchInput={searchInput} />
            </button>
          )}

          <div className="flex items-center gap-1">
            {node.isExpanded && !searchInput && (
              <div className="flex items-center gap-1 opacity-0 transition-opacity duration-100 group-hover:opacity-100">
                <button
                  disabled={allFoldersAreExpanded}
                  className={`disabled:hover:background-transparent disabled:hover:dark:background-transparent background-(--moss-treeNodeButton-bg) hover:background-(--moss-treeNodeButton-bg-hover) flex size-[22px] cursor-pointer items-center justify-center rounded-[3px] text-(--moss-treeNodeButton-text) disabled:cursor-default disabled:opacity-50 disabled:hover:text-(--moss-treeNodeButton-text)`}
                  onClick={handleExpandAll}
                >
                  <Icon icon="TreeExpandAllIcon" />
                </button>

                <button
                  disabled={allFoldersAreCollapsed}
                  className={`disabled:hover:background-transparent disabled:hover:dark:background-transparent background-(--moss-treeNodeButton-bg) hover:background-(--moss-treeNodeButton-bg-hover) flex size-[22px] cursor-pointer items-center justify-center rounded-[3px] text-(--moss-treeNodeButton-text) disabled:cursor-default disabled:opacity-50 disabled:hover:text-(--moss-treeNodeButton-text)`}
                  onClick={handleCollapseAll}
                >
                  <Icon icon="TreeCollapseAllIcon" />
                </button>
              </div>
            )}

            <DropdownMenu.Root>
              <DropdownMenu.Trigger className="background-(--moss-treeNodeButton-bg) hover:background-(--moss-treeNodeButton-bg-hover) flex size-[22px] cursor-pointer items-center justify-center rounded-[3px] text-(--moss-treeNodeButton-text)">
                <Icon icon="TreeDetailIcon" />
              </DropdownMenu.Trigger>
              <DropdownMenu.Portal>
                <DropdownMenu.Content className="z-30">
                  <DropdownMenu.Item label="Add File" onClick={() => setIsAddingRootFileNode(true)} />
                  <DropdownMenu.Item label="Add Folder" onClick={() => setIsAddingRootFolderNode(true)} />
                  <DropdownMenu.Item label="Rename..." onClick={() => setIsRenamingRootNode(true)} />
                </DropdownMenu.Content>
              </DropdownMenu.Portal>
            </DropdownMenu.Root>
          </div>
          {closestEdge && <DropIndicator edge={closestEdge} gap={0} className="z-10" />}
        </div>

        {shouldRenderChildNodes && !isRootDragging && (
          <Scrollbar className="h-full w-full">
            <ul
              className={cn("h-full w-full", {
                "pb-2": node.childNodes.length > 0 && node.isExpanded,
              })}
            >
              {filteredChildNodes.map((childNode) => (
                <TreeNode
                  parentNode={node}
                  onNodeUpdate={onNodeUpdate}
                  key={childNode.uniqueId}
                  node={childNode}
                  depth={0}
                />
              ))}
              {(isAddingRootFileNode || isAddingRootFolderNode) && (
                <div className={nodeStyle} style={{ paddingLeft: `${depth * nodeOffset + paddingLeft}px` }}>
                  <TestCollectionIcon type={node.type} />
                  <NodeAddForm
                    isFolder={isAddingRootFolderNode}
                    restrictedNames={node.childNodes.map((childNode) => childNode.id)}
                    onSubmit={handleAddFormRootSubmit}
                    onCancel={handleAddFormRootCancel}
                  />
                </div>
              )}
            </ul>
          </Scrollbar>
        )}
      </div>
    );
  }

  return (
    <li ref={dropTargetListRef} className="s">
      {isRenamingNode ? (
        <div className={nodeStyle} style={{ paddingLeft: nodePaddingLeft }}>
          <TestCollectionIcon type={node.type} />

          <NodeRenamingForm
            onSubmit={handleRenamingFormSubmit}
            onCancel={handleRenamingFormCancel}
            restrictedNames={parentNode.childNodes.map((childNode) => childNode.id)}
            currentName={node.id}
          />
        </div>
      ) : (
        <ContextMenu.Root modal={false}>
          <ContextMenu.Trigger asChild>
            <button
              ref={draggableNodeRef}
              style={{
                paddingLeft: nodePaddingLeft,
                paddingRight,
              }}
              onClick={node.isFolder ? handleFolderClick : undefined}
              className={cn(
                nodeStyle,
                "background-(--moss-treeNode-bg) focus-within:background-(--moss-treeNode-bg) relative w-full cursor-pointer items-center gap-1 dark:hover:text-black",
                {
                  "hover:background-(--moss-treeNode-bg-hover)": !isNodeDragging,
                }
              )}
            >
              <TestCollectionIcon type={node.type} />

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
                  <ul className="background-(--moss-treeNode-bg-focus)">
                    <TreeNode
                      parentNode={{
                        uniqueId: "-",
                        childNodes: [],
                        type: "",
                        order: 0,
                        isFolder: false,
                        isExpanded: false,
                        isRoot: false,
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
            <ContextMenu.Content>
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
          style={{ paddingLeft: `${(depth + 1) * nodeOffset + paddingLeft}px` }}
          className="flex w-full min-w-0 items-center gap-1"
        >
          <TestCollectionIcon type={node.type} className="ml-auto" />
          <NodeAddForm
            isFolder={isAddingFolderNode}
            restrictedNames={node.childNodes.map((childNode) => childNode.id)}
            onSubmit={handleAddFormSubmit}
            onCancel={handleAddFormCancel}
          />
        </div>
      )}

      {shouldRenderChildNodes && (
        <div className="contents" ref={dropTargetFolderRef}>
          <ul className="h-full">
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
        </div>
      )}
    </li>
  );
};

export default TreeNode;

const TestCollectionIcon = ({ type, className }: { type: string; className?: string }) => {
  switch (type) {
    case "folder":
      return (
        <svg
          className={cn(className, "min-h-4 min-w-4")}
          width="16"
          height="16"
          viewBox="0 0 16 16"
          fill="none"
          xmlns="http://www.w3.org/2000/svg"
        >
          <path
            d="M8.10584 4.34613L8.25344 4.5H8.46667H13C13.8284 4.5 14.5 5.17157 14.5 6V12.1333C14.5 12.9529 13.932 13.5 13.3667 13.5H2.63333C2.06804 13.5 1.5 12.9529 1.5 12.1333V3.86667C1.5 3.04707 2.06804 2.5 2.63333 2.5H6.1217C6.25792 2.5 6.38824 2.55557 6.48253 2.65387L8.10584 4.34613Z"
            fill="#EBECF0"
            stroke="#6C707E"
          />
        </svg>
      );

    case "hdr":
      return (
        <svg
          className={cn(className, "min-h-4 min-w-4")}
          width="16"
          height="16"
          viewBox="0 0 16 16"
          fill="none"
          xmlns="http://www.w3.org/2000/svg"
        >
          <rect width="16" height="16" fill="white" fillOpacity="0.01" />
          <path
            d="M15 7.5V5.5C15 5.23478 14.8946 4.98043 14.7071 4.79289C14.5196 4.60536 14.2652 4.5 14 4.5H11V11.5H12V8.5H12.74L13.91 11.5H15L13.835 8.5H14C14.2652 8.5 14.5196 8.39464 14.7071 8.20711C14.8946 8.01957 15 7.76522 15 7.5ZM12 5.5H14V7.5H12V5.5Z"
            fill="#DB3B4B"
          />
          <path
            d="M8 11.5H6V4.5H8C8.53043 4.5 9.03914 4.71071 9.41421 5.08579C9.78929 5.46086 10 5.96957 10 6.5V9.5C10 10.0304 9.78929 10.5391 9.41421 10.9142C9.03914 11.2893 8.53043 11.5 8 11.5ZM7 10.5H8C8.26522 10.5 8.51957 10.3946 8.70711 10.2071C8.89464 10.0196 9 9.76522 9 9.5V6.5C9 6.23478 8.89464 5.98043 8.70711 5.79289C8.51957 5.60536 8.26522 5.5 8 5.5H7V10.5Z"
            fill="#DB3B4B"
          />
          <path d="M4 4.5V7.5H2V4.5H1V11.5H2V8.5H4V11.5H5V4.5H4Z" fill="#DB3B4B" />
        </svg>
      );

    default:
      return (
        <svg
          className={cn(className, "min-h-4 min-w-4")}
          width="16"
          height="16"
          viewBox="0 0 16 16"
          fill="none"
          xmlns="http://www.w3.org/2000/svg"
        >
          <rect width="16" height="16" fill="white" fillOpacity="0.01" />
          <path
            d="M15 11.5H12C11.7349 11.4997 11.4807 11.3942 11.2932 11.2068C11.1058 11.0193 11.0003 10.7651 11 10.5V5.5C11.0003 5.23486 11.1057 4.98066 11.2932 4.79319C11.4807 4.60571 11.7349 4.50026 12 4.5H15V5.5H12V10.5H15V11.5Z"
            fill="#208A3C"
          />
          <path
            d="M9 11.5H7C6.73488 11.4997 6.4807 11.3942 6.29323 11.2068C6.10576 11.0193 6.0003 10.7651 6 10.5V5.5C6.00026 5.23486 6.10571 4.98066 6.29319 4.79319C6.48066 4.60571 6.73486 4.50026 7 4.5H9C9.26513 4.50026 9.51934 4.60571 9.70681 4.79319C9.89429 4.98066 9.99973 5.23486 10 5.5V10.5C9.9997 10.7651 9.89424 11.0193 9.70677 11.2068C9.5193 11.3942 9.26512 11.4997 9 11.5ZM7 5.5V10.5H9V5.5H7Z"
            fill="#208A3C"
          />
          <path
            d="M3 11.5H1V4.5H3C3.53025 4.5006 4.03861 4.7115 4.41356 5.08644C4.7885 5.46139 4.9994 5.96975 5 6.5V9.5C4.9994 10.0303 4.7885 10.5386 4.41356 10.9136C4.03861 11.2885 3.53025 11.4994 3 11.5ZM2 10.5H3C3.26514 10.4997 3.51934 10.3943 3.70681 10.2068C3.89429 10.0193 3.99974 9.76514 4 9.5V6.5C3.99974 6.23486 3.89429 5.98066 3.70681 5.79319C3.51934 5.60571 3.26514 5.50026 3 5.5H2V10.5Z"
            fill="#208A3C"
          />
        </svg>
      );
  }
};
