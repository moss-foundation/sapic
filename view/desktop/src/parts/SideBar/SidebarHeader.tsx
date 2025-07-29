import { ActionButton, ActionMenu } from "@/components";
import { CreateCollectionModal } from "@/components/Modals/Collection/CreateCollectionModal";
import {
  USE_STREAMED_COLLECTION_ENTRIES_QUERY_KEY,
  useClearAllCollectionEntries,
  useModal,
  useStreamedCollections,
  useStreamedCollectionsWithEntries,
  useWorkspaceSidebarState,
} from "@/hooks";
import { useBatchUpdateCollectionEntry } from "@/hooks/collection/useBatchUpdateCollectionEntry";
import { EntryInfo } from "@repo/moss-collection";
import { useQueryClient } from "@tanstack/react-query";

export const SidebarHeader = ({ title }: { title: string }) => {
  const queryClient = useQueryClient();

  const { isLoading: isCollectionsLoading, clearCollectionsCacheAndRefetch } = useStreamedCollections();
  const { clearAllCollectionEntriesCache } = useClearAllCollectionEntries();
  const { data: collectionsWithEntries } = useStreamedCollectionsWithEntries();
  const { mutateAsync: batchUpdateCollectionEntry } = useBatchUpdateCollectionEntry();
  const { hasWorkspace } = useWorkspaceSidebarState();

  const {
    showModal: showCreateCollectionModal,
    closeModal: closeCreateCollectionModal,
    openModal: openCreateCollectionModal,
  } = useModal();

  const handleRefreshCollections = () => {
    clearCollectionsCacheAndRefetch();
    clearAllCollectionEntriesCache();
  };

  const areAllNodesCollapsed = collectionsWithEntries.every((collection) => {
    return collection.entries.filter((entry) => entry.kind === "Dir").every((entry) => !entry.expanded);
  });

  const handleCollapseAll = async () => {
    if (areAllNodesCollapsed) {
      return;
    }

    const collectionWithEntriesToCollapse = collectionsWithEntries.map((collection) => {
      const entriesToCollapse = collection.entries.filter((entry) => {
        return entry.kind === "Dir" && entry.expanded;
      });

      return {
        collectionId: collection.id,
        entries: entriesToCollapse,
      };
    });

    const promises = collectionWithEntriesToCollapse.map(async (collection) => {
      const preparedEntries = collection.entries.map((entry) => {
        return {
          DIR: {
            id: entry.id,
            expanded: false,
          },
        };
      });

      if (preparedEntries.length > 0) {
        const res = await batchUpdateCollectionEntry({
          collectionId: collection.collectionId,
          entries: {
            entries: preparedEntries,
          },
        });

        if (res.status === "ok") {
          queryClient.setQueryData(
            [USE_STREAMED_COLLECTION_ENTRIES_QUERY_KEY, collection.collectionId],
            (old: EntryInfo[]) => {
              return old.map((entry) => {
                if (preparedEntries.some((preparedEntry) => preparedEntry.DIR.id === entry.id)) {
                  return { ...entry, expanded: false };
                }
                return entry;
              });
            }
          );
        }
      }
    });

    await Promise.all(promises);
  };

  return (
    <div className="background-(--moss-secondary-background) relative flex items-center justify-between px-2 py-[5px] text-(--moss-primary-text) uppercase">
      <div className="w-max items-center overflow-hidden text-xs text-ellipsis whitespace-nowrap text-(--moss-secondary-text)">
        {title}
      </div>

      <div className="flex grow justify-end">
        <ActionButton disabled={!hasWorkspace} icon="Add" onClick={openCreateCollectionModal} />
        <ActionButton disabled={!hasWorkspace || areAllNodesCollapsed} icon="CollapseAll" onClick={handleCollapseAll} />
        <ActionButton disabled={!hasWorkspace} icon="Import" />
        <ActionButton
          icon="Refresh"
          onClick={handleRefreshCollections}
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
