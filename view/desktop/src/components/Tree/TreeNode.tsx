import { SVGProps, useEffect, useRef, useState } from "react";

import { cn } from "@/utils";

import { ContextMenu } from "..";
import RecursiveTree from "./RecursiveTree";
import { NodeProps, TreeNodeProps } from "./types";

export const TreeNode = ({
  node,
  onNodeUpdate,
  onNodeExpand,
  onNodeCollapse,
  depth,
  horizontalPadding,
  nodeOffset,
}: TreeNodeProps) => {
  const paddingLeft = `${depth * nodeOffset + horizontalPadding}px`;
  const paddingRight = `${horizontalPadding}px`;

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

  const handleChildNodesUpdate = (nodes: NodeProps[]) => {
    onNodeUpdate({ ...node, childNodes: nodes });
  };

  const handleKeyUp = (e: React.KeyboardEvent<HTMLButtonElement>) => {
    if (e.key === "F2" && document.activeElement === ref.current) {
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
    <li key={node.id} className={cn("w-full")}>
      {redacting ? (
        <div
          className={cn(
            "flex w-full items-center gap-1 focus-within:bg-[#ebecf0] dark:focus-within:bg-[#434343] text-ellipsis whitespace-nowrap"
          )}
          style={{ paddingLeft, paddingRight }}
        >
          {node.isFolder ? <FolderIcon className="min-w-4 min-h-4" /> : <FileIcon className="min-w-4 min-h-4" />}
          <form onSubmit={handleSubmit} className="grow w-full">
            <input ref={inputRef} className="w-full focus-within:outline-none " onBlur={handleSubmit} />
          </form>

          <ChevronRightIcon className="opacity-0 ml-auto" />
        </div>
      ) : (
        <ContextMenu.Root>
          <ContextMenu.Trigger asChild>
            <button
              className={cn(
                "flex gap-1 w-full grow items-center cursor-pointer focus-within:outline-none focus-within:bg-[#ebecf0] dark:focus-within:bg-[#434343] text-ellipsis whitespace-nowrap"
              )}
              style={{ paddingLeft, paddingRight }}
              onClick={handleClick}
              onKeyUp={handleKeyUp}
              ref={ref}
            >
              {node.isFolder ? <FolderIcon className="min-w-4 min-h-4" /> : <FileIcon className="min-w-4 min-h-4" />}
              <span>{node.name}</span>

              <ChevronRightIcon
                className={cn("ml-auto", {
                  "rotate-90": node.isExpanded,
                  "opacity-0": !node.isFolder,
                })}
              />
            </button>
          </ContextMenu.Trigger>

          <ContextMenu.Content>
            <ContextMenu.Item label="Edit" onClick={() => setRedacting(true)} />
          </ContextMenu.Content>
        </ContextMenu.Root>
      )}
      {node.childNodes && node.isExpanded && (
        <RecursiveTree
          nodes={node.childNodes}
          onChildNodesUpdate={handleChildNodesUpdate}
          onNodeUpdate={onNodeUpdate}
          onNodeExpand={onNodeExpand}
          onNodeCollapse={onNodeCollapse}
          depth={depth + 1}
          horizontalPadding={horizontalPadding}
          nodeOffset={nodeOffset}
        />
      )}
    </li>
  );
};

export default TreeNode;

const FolderIcon = ({ ...props }: SVGProps<SVGSVGElement>) => {
  return (
    <svg width="16" height="16" viewBox="0 0 16 16" fill="none" xmlns="http://www.w3.org/2000/svg" {...props}>
      <path
        d="M8.10584 4.34613L8.25344 4.5H8.46667H13C13.8284 4.5 14.5 5.17157 14.5 6V12.1333C14.5 12.9529 13.932 13.5 13.3667 13.5H2.63333C2.06804 13.5 1.5 12.9529 1.5 12.1333V3.86667C1.5 3.04707 2.06804 2.5 2.63333 2.5H6.1217C6.25792 2.5 6.38824 2.55557 6.48253 2.65387L8.10584 4.34613Z"
        fill="#EBECF0"
        stroke="#6C707E"
      />
    </svg>
  );
};

const FileIcon = ({ ...props }: SVGProps<SVGSVGElement>) => {
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
      {...props}
    >
      <path d="M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7Z" />
      <path d="M14 2v4a2 2 0 0 0 2 2h4" />
    </svg>
  );
};

const ChevronRightIcon = ({ ...props }: SVGProps<SVGSVGElement>) => {
  return (
    <svg width="16" height="16" viewBox="0 0 16 16" fill="none" xmlns="http://www.w3.org/2000/svg" {...props}>
      <path d="M6 11.5L9.5 8L6 4.5" stroke="#818594" strokeLinecap="round" />
    </svg>
  );
};
