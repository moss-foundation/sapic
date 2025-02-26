import { SVGProps } from "react";

import { cn } from "@/utils";

import Tree, { ITreeNode } from "./Tree";

export const TreeNode = ({
  node,
  onNodeUpdate,
  onNodeExpand,
  onNodeCollapse,
  depth,
}: {
  node: ITreeNode;
  onNodeUpdate: (node: ITreeNode) => void;
  onNodeExpand?: (node: ITreeNode) => void;
  onNodeCollapse?: (node: ITreeNode) => void;
  depth: number;
}) => {
  const handleClick = () => {
    if (!node.isFolder) return;
    const newExpanded = !node.isExpanded;
    if (newExpanded) {
      onNodeExpand?.(node);
    } else {
      onNodeCollapse?.(node);
    }
    const updatedItem = { ...node, isExpanded: newExpanded };
    onNodeUpdate(updatedItem);
  };

  const handleChildNodesUpdate = (nodes: ITreeNode[]) => {
    onNodeUpdate({ ...node, childNodes: nodes });
  };

  return (
    <li key={node.id}>
      <button
        className="flex gap-1 items-center cursor-pointer focus-within:outline-1 focus-within:outline-amber-400"
        onClick={handleClick}
      >
        <ChevronRightIcon
          className={cn("", {
            "rotate-90": node.isExpanded,
            "opacity-0": !node.isFolder,
          })}
        />
        {node.isFolder ? <FolderIcon /> : <FileIcon />}
        <span>{node.name}</span>
      </button>
      {node.childNodes && node.isExpanded && (
        <Tree
          nodes={node.childNodes}
          depth={depth + 1}
          onChildNodesUpdate={handleChildNodesUpdate}
          onNodeUpdate={onNodeUpdate}
          onNodeExpand={onNodeExpand}
          onNodeCollapse={onNodeCollapse}
        />
      )}
    </li>
  );
};

export default TreeNode;

const FolderIcon = () => {
  return (
    <svg
      xmlns="http://www.w3.org/2000/svg"
      width="16"
      height="16"
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      strokeWidth="2"
      strokeLinecap="round"
      strokeLinejoin="round"
    >
      <path d="M20 20a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.9a2 2 0 0 1-1.69-.9L9.6 3.9A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13a2 2 0 0 0 2 2Z" />
    </svg>
  );
};

const FileIcon = () => {
  return (
    <svg
      xmlns="http://www.w3.org/2000/svg"
      width="16"
      height="16"
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      strokeWidth="2"
      strokeLinecap="round"
      strokeLinejoin="round"
    >
      <path d="M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7Z" />
      <path d="M14 2v4a2 2 0 0 0 2 2h4" />
    </svg>
  );
};

const ChevronRightIcon = ({ ...props }: SVGProps<SVGSVGElement>) => {
  return (
    <svg
      {...props}
      xmlns="http://www.w3.org/2000/svg"
      width="16"
      height="16"
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      strokeWidth="2"
      strokeLinecap="round"
      strokeLinejoin="round"
    >
      <path d="M9 5l7 7-7 7" />
    </svg>
  );
};
