import { useEffect, useState } from "react";

import { ActionButton, ActionMenu } from "@/components";
import { CreateCollectionModal } from "@/components/Modals/Collection/CreateCollectionModal";
import { DeleteCollectionModal } from "@/components/Modals/Collection/DeleteCollectionModal";
import { useCreateCollectionEntry } from "@/hooks/collection/createCollectionEntry";
import { useCollectionsStore } from "@/store/collections";
import { StreamCollectionsEvent } from "@repo/moss-workspace";
import { Channel, invoke } from "@tauri-apps/api/core";

export const SidebarHeader = ({ title }: { title: string }) => {
  const { collapseAll } = useCollectionsStore();

  const [showCreateCollectionModal, setShowCreateCollectionModal] = useState(false);
  const [showDeleteCollectionModal, setShowDeleteCollectionModal] = useState(false);
  const [streamedCollections, setStreamedCollections] = useState<StreamCollectionsEvent[]>([]);

  // Handle stream_collections using Channels
  useEffect(() => {
    let isActive = true;

    const setupStream = async () => {
      try {
        // Create a channel for receiving collection data
        const onCollectionEvent = new Channel<StreamCollectionsEvent>();

        // Set up the message handler
        onCollectionEvent.onmessage = (message) => {
          if (!isActive) return;

          console.log("Received collection data:", message);

          // Update streamed collections state
          setStreamedCollections((prev) => {
            // Check if collection already exists, update it or add new one
            const existingIndex = prev.findIndex((col) => col.id === message.id);

            if (existingIndex >= 0) {
              // Update existing collection
              const updated = [...prev];
              updated[existingIndex] = message;
              return updated;
            } else {
              // Add new collection
              return [...prev, message];
            }
          });
        };

        // Start streaming collections
        await invoke("stream_collections", {
          channel: onCollectionEvent,
        });
      } catch (error) {
        console.error("Failed to set up stream_collections:", error);
      }
    };

    setupStream();

    return () => {
      isActive = false;
    };
  }, []);

  // Log streamed collections when they change (for debugging)
  useEffect(() => {
    if (streamedCollections.length > 0) {
      console.log("Streamed collections updated:", streamedCollections);

      // Optionally update the main collections store with streamed data
      // You can implement this based on your application's needs
      // For example, merge with existing collections or replace them
    }
  }, [streamedCollections]);

  // Function to manually trigger collection streaming
  const refreshCollections = async () => {
    try {
      const onCollectionEvent = new Channel<StreamCollectionsEvent>();

      onCollectionEvent.onmessage = (message) => {
        console.log("Manual refresh - received collection:", message);
        setStreamedCollections((prev) => {
          const existingIndex = prev.findIndex((col) => col.id === message.id);
          if (existingIndex >= 0) {
            const updated = [...prev];
            updated[existingIndex] = message;
            return updated;
          } else {
            return [...prev, message];
          }
        });
      };

      await invoke("stream_collections", {
        channel: onCollectionEvent,
      });
    } catch (error) {
      console.error("Failed to refresh collections:", error);
    }
  };

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
        <ActionButton icon="Refresh" onClick={refreshCollections} title="Refresh Collections" />
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
      collectionId: "14e13201-c337-4cbe-9d46-156fc141221b",
      input: {
        dir: {
          path: "/requests",
          name: "this should have been a test request",
          configuration: {
            "request": {
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
