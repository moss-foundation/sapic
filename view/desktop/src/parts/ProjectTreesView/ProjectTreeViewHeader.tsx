import { useState } from "react";

import { ActionButton, ActionMenu, SidebarHeader } from "@/components";
import { CREATE_TAB, IMPORT_TAB } from "@/components/Modals/Project/NewProjectModal/constansts";
import { NewProjectModal } from "@/components/Modals/Project/NewProjectModal/NewProjectModal";
import {
  USE_STREAM_COLLECTION_ENTRIES_QUERY_KEY,
  useActiveWorkspace,
  useClearAllCollectionEntries,
  useModal,
  useStreamCollections,
  useStreamedCollectionsWithEntries,
} from "@/hooks";
import { useBatchUpdateCollection } from "@/hooks/collection/useBatchUpdateCollection";
import { useBatchUpdateCollectionEntry } from "@/hooks/collection/useBatchUpdateCollectionEntry";
import { StreamEntriesEvent } from "@repo/moss-project";
import { useQueryClient } from "@tanstack/react-query";

export const CollectionTreeViewHeader = () => {
  const queryClient = useQueryClient();

  const { isLoading: isCollectionsLoading, clearCollectionsCacheAndRefetch } = useStreamCollections();
  const { clearAllCollectionEntriesCache } = useClearAllCollectionEntries();
  const { data: collectionsWithEntries } = useStreamedCollectionsWithEntries();
  const { mutateAsync: batchUpdateCollection } = useBatchUpdateCollection();
  const { mutateAsync: batchUpdateCollectionEntry } = useBatchUpdateCollectionEntry();
  const { hasActiveWorkspace } = useActiveWorkspace();

  const [initialTab, setInitialTab] = useState<typeof CREATE_TAB | typeof IMPORT_TAB>(CREATE_TAB);

  const {
    showModal: showNewProjectModal,
    closeModal: closeNewProjectModal,
    openModal: openNewProjectModal,
  } = useModal();

  const handleRefreshProjects = () => {
    clearCollectionsCacheAndRefetch();
    clearAllCollectionEntriesCache();
  };

  const areAllProjectsCollapsed = collectionsWithEntries.every((collection) => !collection.expanded);
  const areAllDirNodesCollapsed = collectionsWithEntries.every((collection) => {
    return collection.entries.filter((entry) => entry.kind === "Dir").every((entry) => !entry.expanded);
  });

  const handleCollapseAll = async () => {
    await collapseExpandedProjects();
    await collapseExpandedDirEntries();
  };

  const collapseExpandedProjects = async () => {
    const openedProjects = collectionsWithEntries.filter((collection) => collection.expanded);

    if (openedProjects.length === 0) return;

    await batchUpdateCollection({
      items: openedProjects.map((collection) => ({
        id: collection.id,
        expanded: false,
      })),
    });
  };

  const collapseExpandedDirEntries = async () => {
    const projectsWithExpandedDirs = collectionsWithEntries
      .map((collection) => ({
        collectionId: collection.id,
        entries: collection.entries.filter((entry) => entry.kind === "Dir" && entry.expanded),
      }))
      .filter((collection) => collection.entries.length > 0);

    if (projectsWithExpandedDirs.length === 0) return;

    const promises = projectsWithExpandedDirs.map(async (collection) => {
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
          [USE_STREAM_COLLECTION_ENTRIES_QUERY_KEY, collection.collectionId],
          (old: StreamEntriesEvent[]) => {
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
        title="Projects"
        actionsContent={
          <>
            <ActionButton
              title="Add Project"
              disabled={!hasActiveWorkspace}
              icon="Add"
              onClick={() => {
                setInitialTab(CREATE_TAB);
                openNewProjectModal();
              }}
            />
            <ActionButton
              title="Collapse all Projects"
              disabled={!hasActiveWorkspace || (areAllDirNodesCollapsed && areAllProjectsCollapsed)}
              icon="CollapseAll"
              onClick={handleCollapseAll}
            />
            <ActionButton
              title="Import Project"
              disabled={!hasActiveWorkspace}
              icon="Import"
              onClick={() => {
                setInitialTab(IMPORT_TAB);
                openNewProjectModal();
              }}
            />
            <ActionButton
              icon="Refresh"
              onClick={handleRefreshProjects}
              title="Refresh Projects"
              disabled={isCollectionsLoading || !hasActiveWorkspace}
            />

            <PlaceholderDropdownMenu />
          </>
        }
      />
      {showNewProjectModal && (
        <NewProjectModal initialTab={initialTab} showModal={showNewProjectModal} closeModal={closeNewProjectModal} />
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
