import { useContext } from "react";

import { ActionMenu } from "@/components";
import ActionButton from "@/components/ActionButton";
import { DeleteCollectionModal } from "@/components/Modals/Collection/DeleteCollectionModal";
import { useModal, useUpdateCollection } from "@/hooks";
import { Tree } from "@/lib/ui/Tree";
import { useTabbedPaneStore } from "@/store/tabbedPane";

import { useRefreshCollection } from "../actions/useRefreshCollection";
import { useToggleAllNodes } from "../actions/useToggleAllNodes";
import { CollectionTreeContext } from "../CollectionTreeContext";
import { TreeCollectionRootNode } from "../types";

interface TreeRootControlsProps {
  node: TreeCollectionRootNode;
  setIsAddingRootFileNode: (isAdding: boolean) => void;
  setIsAddingRootFolderNode: (isAdding: boolean) => void;
  setIsRenamingRootNode: (isRenaming: boolean) => void;
}

export const TreeRootControls = ({
  node,
  setIsAddingRootFileNode,
  setIsAddingRootFolderNode,
  setIsRenamingRootNode,
}: TreeRootControlsProps) => {
  const { allFoldersAreExpanded, allFoldersAreCollapsed, id, displayMode, showOrders } =
    useContext(CollectionTreeContext);

  const { addOrFocusPanel } = useTabbedPaneStore();

  const { mutateAsync: updateCollection } = useUpdateCollection();
  const { expandAllNodes, collapseAllNodes } = useToggleAllNodes(node);
  const { refreshCollection } = useRefreshCollection(id);

  const { showModal: showDeleteCollectionModal, setShowModal: setShowDeleteCollectionModal } = useModal();

  const handleRefresh = () => {
    refreshCollection();
  };

  const handleIconClick = async (e: React.MouseEvent<HTMLButtonElement>) => {
    e.stopPropagation();

    await updateCollection({
      id,
      expanded: !node.expanded,
    });
  };

  const handleLabelClick = async () => {
    if (!node.expanded) {
      await updateCollection({
        id,
        expanded: true,
      });
    }

    addOrFocusPanel({
      id,
      title: node.name,
      component: "CollectionSettings",
      params: {
        collectionId: id,
        iconType: "Collection",
      },
    });
  };

  return (
    <>
      <Tree.RootNodeControls>
        <Tree.RootNodeTriggers className="overflow-hidden">
          <Tree.RootNodeIcon handleIconClick={handleIconClick} isFolderExpanded={node.expanded} />
          {showOrders && <Tree.RootNodeOrder order={node.order} />}
          <Tree.RootNodeLabel label={node.name} onClick={handleLabelClick} />
        </Tree.RootNodeTriggers>

        <Tree.RootNodeActions>
          <Tree.ActionLabel>main</Tree.ActionLabel>

          <Tree.HoverActions>
            <ActionButton
              customHoverBackground="hover:background-(--moss-icon-primary-background-hover)"
              icon="CollapseAll"
              disabled={allFoldersAreCollapsed}
              onClick={collapseAllNodes}
            />
          </Tree.HoverActions>

          <Tree.PersistentActions>
            {displayMode === "REQUEST_FIRST" && (
              <ActionButton
                customHoverBackground="hover:background-(--moss-icon-primary-background-hover)"
                icon="Add"
                onClick={() => setIsAddingRootFileNode(true)}
              />
            )}
            <ActionMenu.Root>
              <ActionMenu.Trigger asChild>
                <ActionButton
                  customHoverBackground="hover:background-(--moss-icon-primary-background-hover)"
                  icon="MoreHorizontal"
                />
              </ActionMenu.Trigger>
              <ActionMenu.Portal>
                <ActionMenu.Content className="z-40" align="center">
                  <ActionMenu.Item alignWithIcons onClick={() => setIsAddingRootFileNode(true)}>
                    Add File
                  </ActionMenu.Item>
                  <ActionMenu.Item alignWithIcons onClick={() => setIsAddingRootFolderNode(true)}>
                    Add Folder
                  </ActionMenu.Item>
                  <ActionMenu.Item alignWithIcons onClick={() => setIsRenamingRootNode(true)}>
                    Rename...
                  </ActionMenu.Item>
                  <ActionMenu.Item alignWithIcons onClick={handleRefresh}>
                    Refresh
                  </ActionMenu.Item>
                  <ActionMenu.Item alignWithIcons onClick={() => setShowDeleteCollectionModal(true)} icon="Trash">
                    Delete
                  </ActionMenu.Item>
                  <ActionMenu.Item
                    alignWithIcons
                    disabled={allFoldersAreExpanded}
                    onClick={expandAllNodes}
                    icon="ExpandAll"
                  >
                    ExpandAll
                  </ActionMenu.Item>
                </ActionMenu.Content>
              </ActionMenu.Portal>
            </ActionMenu.Root>
          </Tree.PersistentActions>
        </Tree.RootNodeActions>
      </Tree.RootNodeControls>

      {showDeleteCollectionModal && (
        <DeleteCollectionModal
          id={node.id}
          showModal={showDeleteCollectionModal}
          closeModal={() => setShowDeleteCollectionModal(false)}
        />
      )}
    </>
  );
};
