import { useEffect } from "react";

import { ActionButton, ActionMenu } from "@/components";
import { CreateCollectionModal } from "@/components/Modals/Collection/CreateCollectionModal";
import { useModal, useWorkspaceSidebarState } from "@/hooks";
import { useCollectionsStore } from "@/store/collections";

export const SidebarHeader = ({ title }: { title: string }) => {
  const { collapseAll } = useCollectionsStore();
  const { areCollectionsStreaming, startCollectionsStream } = useCollectionsStore();
  const { hasWorkspace } = useWorkspaceSidebarState();
  const {
    showModal: showCreateCollectionModal,
    closeModal: closeCreateCollectionModal,
    openModal: openCreateCollectionModal,
  } = useModal();

  useEffect(() => {
    if (hasWorkspace) {
      startCollectionsStream();
    }
  }, [hasWorkspace, startCollectionsStream]);

  return (
    <div className="background-(--moss-secondary-background) relative flex items-center justify-between px-2 py-[5px] text-(--moss-primary-text) uppercase">
      <div className="w-max items-center overflow-hidden text-xs text-ellipsis whitespace-nowrap text-(--moss-secondary-text)">
        {title}
      </div>

      <div className="flex grow justify-end">
        <ActionButton disabled={!hasWorkspace} icon="Add" onClick={openCreateCollectionModal} />
        <ActionButton disabled={!hasWorkspace} icon="CollapseAll" onClick={collapseAll} />
        <ActionButton disabled={!hasWorkspace} icon="Import" />
        <ActionButton
          icon="Refresh"
          onClick={startCollectionsStream}
          title="Refresh Collections"
          disabled={areCollectionsStreaming || !hasWorkspace}
        />
        <ExampleDropdownMenu />
      </div>

      {showCreateCollectionModal && (
        <CreateCollectionModal showModal={showCreateCollectionModal} closeModal={closeCreateCollectionModal} />
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
