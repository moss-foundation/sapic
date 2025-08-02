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
import { useBatchUpdateCollection } from "@/hooks/collection/useBatchUpdateCollection";
import { useBatchUpdateCollectionEntry } from "@/hooks/collection/useBatchUpdateCollectionEntry";
import { EntryInfo } from "@repo/moss-collection";
import { useQueryClient } from "@tanstack/react-query";

export const SidebarHeader = ({ title }: { title: string }) => {
  const queryClient = useQueryClient();

  const { isLoading: isCollectionsLoading, clearCollectionsCacheAndRefetch } = useStreamedCollections();
  const { clearAllCollectionEntriesCache } = useClearAllCollectionEntries();
  const { data: collectionsWithEntries } = useStreamedCollectionsWithEntries();
  const { mutateAsync: batchUpdateCollection } = useBatchUpdateCollection();
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

  const areAllCollectionsCollapsed = collectionsWithEntries.every((collection) => !collection.expanded);
  const areAllDirNodesCollapsed = collectionsWithEntries.every((collection) => {
    return collection.entries.filter((entry) => entry.kind === "Dir").every((entry) => !entry.expanded);
  });

  const handleCollapseAll = async () => {
    await collapseExpandedCollections();
    await collapseExpandedDirEntries();
  };

  const collapseExpandedCollections = async () => {
    const openedCollections = collectionsWithEntries.filter((collection) => collection.expanded);

    if (openedCollections.length === 0) return;

    await batchUpdateCollection({
      items: openedCollections.map((collection) => ({
        id: collection.id,
        expanded: false,
      })),
    });
  };

  const collapseExpandedDirEntries = async () => {
    const collectionsWithExpandedDirs = collectionsWithEntries
      .map((collection) => ({
        collectionId: collection.id,
        entries: collection.entries.filter((entry) => entry.kind === "Dir" && entry.expanded),
      }))
      .filter((collection) => collection.entries.length > 0);

    if (collectionsWithExpandedDirs.length === 0) return;

    const promises = collectionsWithExpandedDirs.map(async (collection) => {
      const preparedEntries = collection.entries.map((entry) => ({
        DIR: {
          id: entry.id,
          expanded: false,
        },
      }));

      const res = await batchUpdateCollectionEntry({
        collectionId: collection.collectionId,
        entries: {
          entries: preparedEntries,
        },
      });

      if (res.status === "ok") {
        updateQueryCache(collection.collectionId, preparedEntries);
      }
    });

    await Promise.all(promises);
  };

  const updateQueryCache = (
    collectionId: string,
    preparedEntries: Array<{ DIR: { id: string; expanded: boolean } }>
  ) => {
    queryClient.setQueryData([USE_STREAMED_COLLECTION_ENTRIES_QUERY_KEY, collectionId], (old: EntryInfo[]) => {
      return old.map((entry) => {
        const shouldCollapse = preparedEntries.some((preparedEntry) => preparedEntry.DIR.id === entry.id);
        return shouldCollapse ? { ...entry, expanded: false } : entry;
      });
    });
  };

  return (
    <div className="background-(--moss-secondary-background) relative flex items-center justify-between px-2 py-[5px] text-(--moss-primary-text) uppercase">
      <div className="w-max items-center overflow-hidden text-xs text-ellipsis whitespace-nowrap text-(--moss-secondary-text)">
        {title}
      </div>

      <div className="flex grow justify-end">
        <ActionButton disabled={!hasWorkspace} icon="Add" onClick={openCreateCollectionModal} />
        <ActionButton
          disabled={!hasWorkspace || (areAllDirNodesCollapsed && areAllCollectionsCollapsed)}
          icon="CollapseAll"
          onClick={handleCollapseAll}
        />
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
