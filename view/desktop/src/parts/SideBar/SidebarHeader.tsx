import { ActionButton, ActionMenu } from "@/components";
import { CreateCollectionModal } from "@/components/Modals/Collection/CreateCollectionModal";
import { useModal, useStreamedCollections, useWorkspaceSidebarState } from "@/hooks";

export const SidebarHeader = ({ title }: { title: string }) => {
  // const { collapseAll } = useCollectionsStore();
  const { isLoading: isCollectionsLoading, clearQueryCacheAndRefetch } = useStreamedCollections();
  const { hasWorkspace } = useWorkspaceSidebarState();

  const {
    showModal: showCreateCollectionModal,
    closeModal: closeCreateCollectionModal,
    openModal: openCreateCollectionModal,
  } = useModal();

  return (
    <div className="background-(--moss-secondary-background) relative flex items-center justify-between px-2 py-[5px] text-(--moss-primary-text) uppercase">
      <div className="w-max items-center overflow-hidden text-xs text-ellipsis whitespace-nowrap text-(--moss-secondary-text)">
        {title}
      </div>

      <div className="flex grow justify-end">
        <ActionButton disabled={!hasWorkspace} icon="Add" onClick={openCreateCollectionModal} />
        <ActionButton disabled={!hasWorkspace} icon="CollapseAll" onClick={undefined} />
        <ActionButton disabled={!hasWorkspace} icon="Import" />
        <ActionButton
          icon="Refresh"
          onClick={clearQueryCacheAndRefetch}
          title="Refresh Collections"
          disabled={isCollectionsLoading || !hasWorkspace}
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
