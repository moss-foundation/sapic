import { useState } from "react";

import { ActionButton, ActionMenu } from "@/components";
import { CreateCollectionModal } from "@/components/Modals/Collection/CreateCollectionModal";
import { DeleteCollectionModal } from "@/components/Modals/Collection/DeleteCollectionModal";
import { useCreateCollectionEntry } from "@/hooks/collection/createCollectionEntry";
import { useCollectionsStore } from "@/store/collections";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";

export const SidebarHeader = ({ title }: { title: string }) => {
  const { collapseAll } = useCollectionsStore();

  const [showCreateCollectionModal, setShowCreateCollectionModal] = useState(false);
  const [showDeleteCollectionModal, setShowDeleteCollectionModal] = useState(false);

  const appWebview = getCurrentWebviewWindow();
  appWebview.listen("stream_collections", (event) => {
    console.log(event.payload);
  });

  // useEffect(() => {
  //   const listenCollections = async () => {
  //     return await listen("stream_collections", (event) => {
  //       console.log(event.payload);
  //     });
  //   };

  //   const unlisten = listenCollections();

  //   return unlisten;
  // }, []);

  // useEffect(() => {
  //   let unlisten: UnlistenFn | undefined;
  //   const setupListener = async () => {
  //     try {
  //       unlisten = await listen("stream_collections", (event) => {
  //         console.log(event.payload);
  //       });
  //     } catch (error) {
  //       console.error("Failed to set up theme change listener:", error);
  //     }
  //   };

  //   setupListener();

  //   return () => {
  //     if (unlisten) {
  //       unlisten();
  //     }
  //   };
  // }, []);

  return (
    <div className="background-(--moss-secondary-background) relative flex items-center justify-between px-2 py-[5px] text-(--moss-primary-text) uppercase">
      <div className="w-max items-center overflow-hidden text-xs text-ellipsis whitespace-nowrap text-(--moss-secondary-text)">
        {title}
      </div>

      <div className="flex grow justify-end">
        <ActionButton icon="Add" onClick={() => setShowCreateCollectionModal(true)} />
        <ActionButton icon="Trash" onClick={() => setShowDeleteCollectionModal(true)} />
        <ActionButton icon="CollapseAll" onClick={collapseAll} />
        <ActionButton icon="Import" />
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
  const { mutateAsync: createCollectionEntry } = useCreateCollectionEntry();

  const handleCreateCollectionEntry = async () => {
    await createCollectionEntry({
      collectionId: "d8bb1244-e552-42e9-be25-1184a370f32a",
      input: {
        dir: {
          path: "/requests",
          name: "this should have been a test request",
          configuration: {
            Request: {
              http: {},
            },
          },
        },
      },
    });
  };

  return (
    <ActionMenu.Root>
      <ActionMenu.Trigger asChild>
        <ActionButton icon="MoreHorizontal" />
      </ActionMenu.Trigger>

      <ActionMenu.Portal>
        <ActionMenu.Content align="center">
          <ActionMenu.Item onSelect={handleCreateCollectionEntry}>createCollectionEntry</ActionMenu.Item>
          <ActionMenu.Item onSelect={() => console.log("Item 2 selected")}>Item 2</ActionMenu.Item>
        </ActionMenu.Content>
      </ActionMenu.Portal>
    </ActionMenu.Root>
  );
};
