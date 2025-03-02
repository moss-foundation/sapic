import { useEffect, useRef, useState } from "react";
import { createPortal } from "react-dom";

import { cn } from "@/utils";
import { draggable, dropTargetForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";
import { setCustomNativeDragPreview } from "@atlaskit/pragmatic-drag-and-drop/element/set-custom-native-drag-preview";

import { ContextMenu } from "..";
import { ChevronRightIcon, FileIcon, FolderIcon } from "./Icons";
import { NodeRenamingForm } from "./NodeRenamingForm";
import { TreeNodeComponentProps } from "./types";
import { canDrop, getActualDropSourceTarget, getActualDropTarget } from "./utils";

export const TreeNode = ({
  node,
  onNodeUpdate,
  depth,
  horizontalPadding,
  nodeOffset,
  treeId,
  parentNode,
}: TreeNodeComponentProps) => {
  const paddingLeft = `${depth * nodeOffset + horizontalPadding}px`;
  const paddingRight = `${horizontalPadding}px`;

  const buttonRef = useRef<HTMLButtonElement>(null);
  const ulList = useRef<HTMLUListElement>(null);
  const spanRef = useRef<HTMLSpanElement>(null);

  const [preview, setPreview] = useState<HTMLElement | null>(null);
  const [dropAllowance, setDropAllowance] = useState<boolean | null>(null);

  const handleFolderClick = () => {
    if (!node.isFolder) return;

    onNodeUpdate({
      ...node,
      isExpanded: !node.isExpanded,
    });
  };

  useEffect(() => {
    const element = buttonRef.current;
    if (!element) return;

    return draggable({
      element,
      getInitialData: () => ({
        type: "TreeNode",
        data: {
          node,
          treeId,
        },
      }),
      onDrop: () => {
        setPreview(null);
      },
      onGenerateDragPreview({ nativeSetDragImage }) {
        setCustomNativeDragPreview({
          nativeSetDragImage,
          render({ container }) {
            setPreview((prev) => (prev === container ? prev : container));
          },
        });
      },
    });
  }, [treeId, dropAllowance, node]);

  useEffect(() => {
    const element = spanRef.current || ulList.current;
    if (!element) return;

    return dropTargetForElements({
      element,
      getData: () => ({
        type: "TreeNode",
        data: {
          treeId,
          node,
        },
      }),
      onDragLeave() {
        setDropAllowance(null);
      },
      onDrag({ location, source }) {
        if (location.current.dropTargets[0].data.type !== "TreeNode" || location.current?.dropTargets.length === 0) {
          return;
        }

        const sourceTarget = getActualDropSourceTarget(source);
        const dropTarget = getActualDropTarget(location);

        if (!dropTarget || !sourceTarget) {
          setDropAllowance(null);
          return;
        }

        if (dropTarget?.node.uniqueId !== node.uniqueId) {
          setDropAllowance(null);
          return;
        }

        setDropAllowance(canDrop(sourceTarget, dropTarget, node));
      },
      onDrop({ location, source }) {
        if (location.current?.dropTargets.length === 0 || location.current.dropTargets[0].data.type !== "TreeNode") {
          return;
        }

        const sourceTarget = getActualDropSourceTarget(source);
        const dropTarget = getActualDropTarget(location);

        if (dropTarget?.node.uniqueId !== node.uniqueId) {
          setDropAllowance(null);
          return;
        }

        if (dropAllowance) {
          window.dispatchEvent(
            new CustomEvent("moveTreeNode", {
              detail: {
                source: sourceTarget,
                target: dropTarget,
              },
            })
          );
        }
        setDropAllowance(null);
      },
    });
  }, [dropAllowance, node, treeId]);

  const [renaming, setRenaming] = useState(false);
  const handleButtonKeyUp = (e: React.KeyboardEvent<HTMLButtonElement>) => {
    if (e.key === "F2" && document.activeElement === e.currentTarget) {
      setRenaming(true);
    }
  };

  const handleFormSubmit = (newId: string) => {
    onNodeUpdate({ ...node, id: newId });
    setRenaming(false);
  };
  const handleFormCancel = () => {
    setRenaming(false);
  };

  if (node.id === "root") {
    return (
      <ul
        ref={ulList}
        className={cn({
          "bg-green-600": dropAllowance === true,
          "bg-red-600": dropAllowance === false,
          "": dropAllowance === null,
        })}
      >
        {node.childNodes.map((childNode) => {
          return (
            <TreeNode
              parentNode={node}
              treeId={treeId}
              onNodeUpdate={onNodeUpdate}
              key={childNode.uniqueId}
              node={childNode}
              depth={0}
              horizontalPadding={horizontalPadding}
              nodeOffset={nodeOffset}
            />
          );
        })}
      </ul>
    );
  }

  return (
    <li
      className={cn({
        "bg-green-600": dropAllowance === true,
        "bg-red-600": dropAllowance === false,
        "": dropAllowance === null,
      })}
    >
      <span className="DropCapture" ref={spanRef}>
        {renaming ? (
          <div
            className="flex w-full min-w-0 items-center gap-1 focus-within:bg-[#ebecf0] dark:focus-within:bg-[#434343]"
            style={{ paddingLeft }}
          >
            {node.isFolder ? <FolderIcon className="min-w-4 min-h-4" /> : <FileIcon className="min-w-4 min-h-4" />}
            <NodeRenamingForm
              onSubmit={handleFormSubmit}
              onCancel={handleFormCancel}
              restrictedNames={parentNode.childNodes.map((childNode) => childNode.id)}
              currentName={node.id}
            />
          </div>
        ) : (
          <ContextMenu.Root modal={false}>
            <ContextMenu.Trigger asChild>
              <button
                ref={buttonRef}
                onClick={node.isFolder ? handleFolderClick : undefined}
                onKeyUp={handleButtonKeyUp}
                style={{ paddingLeft, paddingRight }}
                className="flex gap-1 w-full min-w-0 grow items-center cursor-pointer focus-within:outline-none focus-within:bg-[#ebecf0] dark:focus-within:bg-[#747474] relative"
              >
                {node.isFolder ? <FolderIcon className="min-w-4 min-h-4" /> : <FileIcon className="min-w-4 min-h-4" />}

                <span className="text-ellipsis whitespace-nowrap w-max overflow-hidden">{node.id}</span>

                <span className="DragHandle h-full min-h-4 grow" />

                <ChevronRightIcon
                  className={cn("ml-auto min-w-4 min-h-4", {
                    "rotate-90": node.isExpanded,
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
                        treeId={treeId}
                        node={{ ...node, childNodes: [] }}
                        onNodeUpdate={() => {}}
                        depth={0}
                        horizontalPadding={0}
                        nodeOffset={0}
                      />
                    </ul>,
                    preview
                  )}
              </button>
            </ContextMenu.Trigger>

            <ContextMenu.Portal>
              <ContextMenu.Content className="text-white">
                <ContextMenu.Item label="Edit" onClick={() => setRenaming(true)} />
                <ContextMenu.Item label="Item 2" />
                <ContextMenu.Item label="Item 3" />
              </ContextMenu.Content>
            </ContextMenu.Portal>
          </ContextMenu.Root>
        )}

        {node.isFolder && node.isExpanded && (
          <ul ref={ulList}>
            {node.childNodes.map((childNode) => {
              return (
                <TreeNode
                  parentNode={node}
                  treeId={treeId}
                  key={childNode.uniqueId}
                  node={childNode}
                  depth={depth + 1}
                  onNodeUpdate={onNodeUpdate}
                  horizontalPadding={horizontalPadding}
                  nodeOffset={nodeOffset}
                />
              );
            })}
          </ul>
        )}
      </span>
    </li>
  );
};

export default TreeNode;
