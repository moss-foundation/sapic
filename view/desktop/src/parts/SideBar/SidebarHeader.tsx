import { useEffect, useState } from "react";

import { ActionButton, ActionMenu } from "@/components";
import { CreateCollectionModal } from "@/components/Modals/Collection/CreateCollectionModal";
import { DeleteCollectionModal } from "@/components/Modals/Collection/DeleteCollectionModal";
import { useCollectionsStore } from "@/store/collections";

export const SidebarHeader = ({ title }: { title: string }) => {
  const { collapseAll } = useCollectionsStore();
  const { streamedCollections, isBeingStreamed, refreshCollections } = useCollectionsStore();

  useEffect(() => {
    refreshCollections();
  }, []);

  const [showCreateCollectionModal, setShowCreateCollectionModal] = useState(false);
  const [showDeleteCollectionModal, setShowDeleteCollectionModal] = useState(false);

  return (
    <div className="background-(--moss-secondary-background) relative flex items-center justify-between px-2 py-[5px] text-(--moss-primary-text) uppercase">
      <div className="w-max items-center overflow-hidden text-xs text-ellipsis whitespace-nowrap text-(--moss-secondary-text)">
        {title}
      </div>

      <div className="flex grow justify-end">
        <div>{isBeingStreamed.toString()}</div>
        <div>{streamedCollections?.length}</div>
        <ActionButton icon="Add" onClick={() => setShowCreateCollectionModal(true)} />
        <ActionButton icon="Trash" onClick={() => setShowDeleteCollectionModal(true)} />
        <ActionButton icon="CollapseAll" onClick={collapseAll} />
        <ActionButton icon="Import" />
        <ActionButton
          icon="Refresh"
          onClick={() => refreshCollections()}
          title="Refresh Collections"
          disabled={isBeingStreamed}
        />
        <ExampleDropdownMenu />
      </div>

      {showCreateCollectionModal && (
        <CreateCollectionModal
          showModal={showCreateCollectionModal}
          closeModal={() => setShowCreateCollectionModal(false)}
        />
      )}

      {showDeleteCollectionModal && (
        <DeleteCollectionModal
          showModal={showDeleteCollectionModal}
          closeModal={() => setShowDeleteCollectionModal(false)}
        />
      )}
    </div>
  );
};

export default SidebarHeader;

const ExampleDropdownMenu = () => {
  return (
    <ActionMenu.Root>
      <ActionMenu.Trigger asChild>
        <ActionButton icon="MoreHorizontal" />
      </ActionMenu.Trigger>

      <ActionMenu.Portal>
        <ActionMenu.Content align="center">
          <ActionMenu.Item onSelect={() => console.log("Item 2 selected")}>Item 2</ActionMenu.Item>
        </ActionMenu.Content>
      </ActionMenu.Portal>
    </ActionMenu.Root>
  );
};
