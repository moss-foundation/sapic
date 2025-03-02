import { useEffect, useRef, useState } from "react";
import { createPortal } from "react-dom";

import { cn } from "@/utils";
import { draggable, dropTargetForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";
import { setCustomNativeDragPreview } from "@atlaskit/pragmatic-drag-and-drop/element/set-custom-native-drag-preview";

import { ChevronRightIcon, FileIcon, FolderIcon } from "./Icons";
import { TreeNodeComponentProps } from "./types";
import { canDrop, getActualDropSourceTarget, getActualDropTarget } from "./utils";

export const TreeNode = ({
  node,
  onNodeUpdate,
  depth,
  horizontalPadding,
  nodeOffset,
  treeId,
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
        <button
          ref={buttonRef}
          onClick={node.isFolder ? handleFolderClick : undefined}
          style={{ paddingLeft, paddingRight }}
          className="flex gap-1 w-full min-w-0 grow items-center cursor-pointer focus-within:outline-none focus-within:bg-[#ebecf0] dark:focus-within:bg-[#747474] relative"
        >
          {node.isFolder ? <FolderIcon className="min-w-4 min-h-4" /> : <FileIcon className="min-w-4 min-h-4" />}
          <span>{node.id}</span>
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

        {node.isFolder && node.isExpanded && (
          <ul ref={ulList}>
            {node.childNodes.map((childNode) => {
              return (
                <TreeNode
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
