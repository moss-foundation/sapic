import { useState } from "react";

import { ActionButton, ActionMenu, SidebarHeader } from "@/components";
import { NewCollectionModal } from "@/components/Modals/Collection/NewCollectionModal/NewCollectionModal";
import {
  USE_STREAMED_COLLECTION_ENTRIES_QUERY_KEY,
  useActiveWorkspace,
  useClearAllCollectionEntries,
  useModal,
  useStreamedCollections,
  useStreamedCollectionsWithEntries,
} from "@/hooks";
import { useBatchUpdateCollection } from "@/hooks/collection/useBatchUpdateCollection";
import { useBatchUpdateCollectionEntry } from "@/hooks/collection/useBatchUpdateCollectionEntry";
import { EntryInfo } from "@repo/moss-collection";
import { useQueryClient } from "@tanstack/react-query";

export const CollectionTreeViewHeader = () => {
  const queryClient = useQueryClient();

  const { isLoading: isCollectionsLoading, clearCollectionsCacheAndRefetch } = useStreamedCollections();
  const { clearAllCollectionEntriesCache } = useClearAllCollectionEntries();
  const { data: collectionsWithEntries } = useStreamedCollectionsWithEntries();
  const { mutateAsync: batchUpdateCollection } = useBatchUpdateCollection();
  const { mutateAsync: batchUpdateCollectionEntry } = useBatchUpdateCollectionEntry();
  const { hasActiveWorkspace } = useActiveWorkspace();

  const [initialTab, setInitialTab] = useState<"Create" | "Import">("Create");

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
        queryClient.setQueryData(
          [USE_STREAMED_COLLECTION_ENTRIES_QUERY_KEY, collection.collectionId],
          (old: EntryInfo[]) => {
            return old.map((entry) => {
              const shouldCollapse = preparedEntries.some((preparedEntry) => preparedEntry.DIR.id === entry.id);
              return shouldCollapse ? { ...entry, expanded: false } : entry;
            });
          }
        );
      }
    });

    await Promise.all(promises);
  };

  return (
    <>
      <SidebarHeader
        title="Collections"
        actionsContent={
          <>
            <ActionButton
              title="Add collection"
              disabled={!hasActiveWorkspace}
              icon="Add"
              onClick={() => {
                setInitialTab("Create");
                openCreateCollectionModal();
              }}
            />
            <ActionButton
              title="Collapse all collections"
              disabled={!hasActiveWorkspace || (areAllDirNodesCollapsed && areAllCollectionsCollapsed)}
              icon="CollapseAll"
              onClick={handleCollapseAll}
            />
            <ActionButton
              title="Import collection"
              disabled={!hasActiveWorkspace}
              icon="Import"
              onClick={() => {
                setInitialTab("Import");
                openCreateCollectionModal();
              }}
            />
            <ActionButton
              icon="Refresh"
              onClick={handleRefreshCollections}
              title="Refresh collections"
              disabled={isCollectionsLoading || !hasActiveWorkspace}
            />

            <PlaceholderDropdownMenu />
          </>
        }
      />
      {showCreateCollectionModal && (
        <NewCollectionModal
          initialTab={initialTab}
          showModal={showCreateCollectionModal}
          closeModal={closeCreateCollectionModal}
        />
      )}
    </>
  );
};

const PlaceholderDropdownMenu = () => {
  return (
    <ActionMenu.Root>
      <ActionMenu.Trigger asChild>
        <ActionButton icon="MoreHorizontal" />
      </ActionMenu.Trigger>

      <ActionMenu.Portal>
        <ActionMenu.Content align="center">
          <ActionMenu.Item onSelect={() => console.log("Item 1 selected")}>Placeholder Item 1</ActionMenu.Item>
          <ActionMenu.Item onSelect={() => console.log("Item 2 selected")}>Placeholder Item 2</ActionMenu.Item>
          <ActionMenu.Item onSelect={() => console.log("Item 3 selected")}>Placeholder Item 3</ActionMenu.Item>
        </ActionMenu.Content>
      </ActionMenu.Portal>
    </ActionMenu.Root>
  );
};
