import { ActionMenu } from "@/components";
import { ActionButton } from "@/components/ActionButton";

import { TreeCollectionRootNode } from "../types";

// interface TreeRootNodeActionsProps {
//   node: TreeNodeProps;
//   searchInput?: string;
//   isRenamingRootNode: boolean;
//   setIsAddingRootFileNode: (isAdding: boolean) => void;
//   setIsAddingRootFolderNode: (isAdding: boolean) => void;
//   setIsRenamingRootNode: (isRenaming: boolean) => void;
//   allFoldersAreCollapsed: boolean;
//   allFoldersAreExpanded: boolean;
//   handleCollapseAll: () => void;
//   handleExpandAll: () => void;
// }

interface TreeRootNodeActionsProps {
  node: TreeCollectionRootNode;
  searchInput?: string;
  isRenamingRootNode: boolean;
  // setIsAddingRootFileNode: (isAdding: boolean) => void;
  // setIsAddingRootFolderNode: (isAdding: boolean) => void;
  setIsRenamingRootNode: (isRenaming: boolean) => void;
  allFoldersAreCollapsed: boolean;
  allFoldersAreExpanded: boolean;
  // handleCollapseAll: () => void;
  // handleExpandAll: () => void;
}

export const TreeRootNodeActions = ({
  node,
  searchInput,
  isRenamingRootNode,
  // setIsAddingRootFileNode,
  // setIsAddingRootFolderNode,
  setIsRenamingRootNode,
  allFoldersAreCollapsed,
  allFoldersAreExpanded,
  // handleCollapseAll,
  // handleExpandAll,
}: TreeRootNodeActionsProps) => {
  return (
    <div className="z-10 flex items-center">
      {node.expanded && !searchInput && !isRenamingRootNode && (
        <div
          className={`hidden items-center opacity-0 transition-[display,opacity] transition-discrete duration-100 group-hover:flex group-hover:opacity-100`}
        >
          <ActionButton
            customHoverBackground="hover:background-(--moss-icon-primary-background-hover)"
            icon="Add"
            // onClick={() => setIsAddingRootFileNode(true)}
          />
          <ActionButton
            customHoverBackground="hover:background-(--moss-icon-primary-background-hover)"
            icon="CollapseAll"
            disabled={allFoldersAreCollapsed}
            // onClick={handleCollapseAll}
          />
        </div>
      )}
      <ActionMenu.Root>
        <ActionMenu.Trigger asChild>
          <ActionButton
            customHoverBackground="hover:background-(--moss-icon-primary-background-hover)"
            icon="MoreHorizontal"
          />
        </ActionMenu.Trigger>
        <ActionMenu.Portal>
          <ActionMenu.Content className="z-30" align="center">
            {/* <ActionMenu.Item onClick={() => setIsAddingRootFileNode(true)}>Add File</ActionMenu.Item>
            <ActionMenu.Item onClick={() => setIsAddingRootFolderNode(true)}>Add Folder</ActionMenu.Item> */}
            <ActionMenu.Item onClick={() => setIsRenamingRootNode(true)}>Rename...</ActionMenu.Item>
            <ActionMenu.Item>Refresh</ActionMenu.Item>
            {/* <ActionMenu.Item disabled={allFoldersAreExpanded} onClick={handleExpandAll}>
                ExpandAll
              </ActionMenu.Item> */}
          </ActionMenu.Content>
        </ActionMenu.Portal>
      </ActionMenu.Root>
    </div>
  );
};
