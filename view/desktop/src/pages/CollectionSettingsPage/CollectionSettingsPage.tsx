import { useState, useEffect } from "react";

import { ConfirmationModal } from "@/components/Modals/ConfirmationModal";
import { useUpdateCollection, useDeleteCollection, useActiveCollection } from "@/hooks/collection";
import { Collection } from "@/components/CollectionTree/types";

import { CollectionNameSection } from "./CollectionNameSection";
import { CollectionSummarySection } from "./CollectionSummarySection";
import { CollectionDangerZoneSection } from "./CollectionDangerZoneSection";

export const CollectionSettings = () => {
  const collection = useActiveCollection();
  const { mutate: updateCollection } = useUpdateCollection();
  const { mutate: deleteCollection } = useDeleteCollection();

  const [name, setName] = useState("");
  const [repository, setRepository] = useState("github.com/moss-foundation/sapic");
  const [showDeleteConfirmModal, setShowDeleteConfirmModal] = useState(false);

  useEffect(() => {
    if (collection) {
      // Get display name from collection
      const displayName = typeof collection.id === "string" ? collection.id : `Collection ${collection.id}`;
      setName(displayName);
    }
  }, [collection]);

  const handleSave = () => {
    if (name.trim() && collection && name.trim() !== getCollectionDisplayName(collection)) {
      const collectionId = typeof collection.id === "string" ? collection.id : String(collection.id);
      updateCollection(
        { collectionId, name: name.trim() },
        {
          onError: (error) => {
            console.error("Failed to update collection:", error.message);
          },
        }
      );
    }
  };

  // Auto-save when input loses focus
  const handleBlur = () => {
    handleSave();
  };

  // Delete collection handlers
  const handleDeleteClick = () => {
    setShowDeleteConfirmModal(true);
  };

  const handleDeleteCollection = () => {
    if (collection) {
      const collectionId = typeof collection.id === "string" ? collection.id : String(collection.id);
      deleteCollection({ id: collectionId });
      setShowDeleteConfirmModal(false);
    }
  };

  const closeDeleteConfirmModal = () => {
    setShowDeleteConfirmModal(false);
  };

  // Helper function to get collection display name
  const getCollectionDisplayName = (collection: Collection) => {
    return typeof collection.id === "string" ? collection.id : `Collection ${collection.id}`;
  };

  if (!collection) {
    return (
      <div className="flex h-full items-center justify-center text-(--moss-primary-text)">
        <div className="text-center">
          <h2 className="text-lg font-semibold">No Active Collection</h2>
          <p className="text-sm">Please select a collection to view its settings.</p>
        </div>
      </div>
    );
  }

  const collectionDisplayName = getCollectionDisplayName(collection);

  return (
    <>
      <ConfirmationModal
        showModal={showDeleteConfirmModal}
        closeModal={closeDeleteConfirmModal}
        title="Delete"
        message={`Delete "${collectionDisplayName}"?`}
        description="This will delete all requests, endpoints, and other items in this collection. This action cannot be undone."
        confirmLabel="Delete"
        cancelLabel="Close"
        onConfirm={handleDeleteCollection}
        variant="danger"
      />

      <div className="relative flex h-full justify-center">
        {/* Main Content - Centered on full page width */}
        <div className="w-full max-w-2xl space-y-9 px-6 py-5">
          <CollectionNameSection
            name={name}
            setName={setName}
            repository={repository}
            setRepository={setRepository}
            onSave={handleSave}
            onBlur={handleBlur}
          />

          <CollectionDangerZoneSection onDeleteClick={handleDeleteClick} />
        </div>

        {/* Right Column - Summary positioned absolutely on the right */}
        <div className="absolute top-0 right-0 w-80 px-6 py-5">
          <CollectionSummarySection />
        </div>
      </div>
    </>
  );
};
