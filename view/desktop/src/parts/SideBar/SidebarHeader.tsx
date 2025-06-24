import { useEffect, useState } from "react";

import { ActionButton, ActionMenu } from "@/components";
import { CreateCollectionModal } from "@/components/Modals/Collection/CreateCollectionModal";
import { useWorkspaceSidebarState } from "@/hooks";
import { useCollectionsStore } from "@/store/collections";

export const SidebarHeader = ({ title }: { title: string }) => {
  const { collapseAll } = useCollectionsStore();
  const { areCollectionsStreaming, startCollectionsStream, createCollectionEntry } = useCollectionsStore();
  const { hasWorkspace } = useWorkspaceSidebarState();

  useEffect(() => {
    if (hasWorkspace) {
      startCollectionsStream();
    }
  }, [hasWorkspace, startCollectionsStream]);

  const [showCreateCollectionModal, setShowCreateCollectionModal] = useState(false);

  const handleCreateCollectionEntry = () => {
    createCollectionEntry({
      collectionId: "7e353d76-8894-4007-a6da-2c96d9951eb7",
      input: {
        item: {
          name: "Test root item",
          path: "/requests",
          configuration: {
            request: {
              http: {
                requestParts: {
                  method: "GET",
                },
              },
            },
          },
        },
      },
    });
  };

  return (
    <div className="background-(--moss-secondary-background) relative flex items-center justify-between px-2 py-[5px] text-(--moss-primary-text) uppercase">
      <div className="w-max items-center overflow-hidden text-xs text-ellipsis whitespace-nowrap text-(--moss-secondary-text)">
        {title}
      </div>

      <div className="flex grow justify-end">
        <ActionButton icon="Add" onClick={() => setShowCreateCollectionModal(true)} />
        <ActionButton icon="CollapseAll" onClick={collapseAll} />
        <ActionButton icon="Import" onClick={handleCreateCollectionEntry} />
        <ActionButton
          icon="Refresh"
          onClick={startCollectionsStream}
          title="Refresh Collections"
          disabled={areCollectionsStreaming}
        />
        <ExampleDropdownMenu />
      </div>

      {showCreateCollectionModal && (
        <CreateCollectionModal
          showModal={showCreateCollectionModal}
          closeModal={() => setShowCreateCollectionModal(false)}
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
