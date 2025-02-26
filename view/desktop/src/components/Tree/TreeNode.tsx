import { SVGProps, useEffect, useRef, useState } from "react";

import { cn } from "@/utils";

import { ContextMenu } from "..";
import Tree, { ITreeNode } from "./Tree";

interface TreeNodeProps {
  node: ITreeNode;
  onNodeUpdate: (node: ITreeNode) => void;
  onNodeExpand?: (node: ITreeNode) => void;
  onNodeCollapse?: (node: ITreeNode) => void;
  depth: number;
}

export const TreeNode = ({ node, onNodeUpdate, onNodeExpand, onNodeCollapse, depth }: TreeNodeProps) => {
  const ref = useRef<HTMLButtonElement>(null);
  const inputRef = useRef<HTMLInputElement>(null);
  const [redacting, setRedacting] = useState(false);

  const handleClick = () => {
    if (!node.isFolder) return;

    const updatedItem = { ...node, isExpanded: !node.isExpanded };

    if (updatedItem.isExpanded) {
      onNodeExpand?.(updatedItem);
    } else {
      onNodeCollapse?.(updatedItem);
    }

    onNodeUpdate(updatedItem);
  };

  const handleChildNodesUpdate = (nodes: ITreeNode[]) => {
    onNodeUpdate({ ...node, childNodes: nodes });
  };

  const handleKeyUp = (e: React.KeyboardEvent<HTMLButtonElement>) => {
    if ((e.key === "F2", document.activeElement === ref.current)) {
      setRedacting(true);
    }
  };

  useEffect(() => {
    if (redacting && inputRef.current) {
      inputRef.current.focus();
      inputRef.current.value = node.name;

      const dotIndex = inputRef.current.value.indexOf(".");
      inputRef.current.setSelectionRange(0, dotIndex);
    }
  }, [redacting, inputRef, node.name]);

  const handleSubmit = (e: React.FormEvent<HTMLFormElement> | React.FocusEvent<HTMLInputElement>) => {
    if ("preventDefault" in e) e.preventDefault();

    const newName = inputRef.current?.value.trim();
    if (newName && newName !== node.name) {
      onNodeUpdate({ ...node, name: newName });
    }
    setRedacting(false);
  };

  return (
    <li key={node.id} className="w-full">
      {redacting ? (
        <div className="flex w-full items-center gap-1 focus-within:bg-gray-800 px-[1px]">
          <ChevronRightIcon className="opacity-0" />
          {node.isFolder ? <FolderIcon /> : <FileIcon />}
          <form onSubmit={handleSubmit} className="grow w-full">
            <input ref={inputRef} className="w-full focus-within:outline-none " onBlur={handleSubmit} />
          </form>
        </div>
      ) : (
        <ContextMenu.Root>
          <ContextMenu.Trigger asChild>
            <button
              className="flex gap-1 w-full grow items-center cursor-pointer focus-within:outline-none focus-within:bg-gray-800 "
              onClick={handleClick}
              onKeyUp={handleKeyUp}
              ref={ref}
            >
              <ChevronRightIcon
                className={cn({
                  "rotate-90": node.isExpanded,
                  "opacity-0": !node.isFolder,
                })}
              />
              {node.isFolder ? <FolderIcon /> : <FileIcon />}
              <span>{node.name}</span>
            </button>
          </ContextMenu.Trigger>

          <ContextMenu.Content>
            <div>
              <table className="border-separate border-spacing-3 ">
                <tbody>
                  <tr>
                    <td>ID</td>
                    <td>{node.id}</td>
                  </tr>
                  <tr>
                    <td>Name</td>
                    <td>{node.name}</td>
                  </tr>
                  <tr>
                    <td>Is Folder</td>
                    <td>{node.isFolder.toString()}</td>
                  </tr>
                </tbody>
              </table>
            </div>
          </ContextMenu.Content>
        </ContextMenu.Root>
      )}
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
