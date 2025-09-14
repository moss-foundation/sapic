import { useBatchUpdateCollection, useDeleteCollection, useStreamCollections } from "@/hooks";
import { useTabbedPaneStore } from "@/store/tabbedPane";

import { ConfirmationModal } from "../ConfirmationModal";
import { ModalWrapperProps } from "../types";

export const DeleteProjectModal = ({ closeModal, showModal, id }: ModalWrapperProps & { id: string }) => {
  const { data: streamedCollections } = useStreamCollections();
  const { mutateAsync: deleteCollection, isPending: isDeleteCollectionLoading } = useDeleteCollection();
  const { mutateAsync: batchUpdateCollection } = useBatchUpdateCollection();

  const { removePanel } = useTabbedPaneStore();

  const collection = streamedCollections?.find((collection) => collection.id === id);

  const handleSubmit = async () => {
    const collectionToDelete = streamedCollections?.find((collection) => collection.id === id);

    if (!collectionToDelete) {
      return;
    }

    try {
      await deleteCollection({ id: collectionToDelete.id });

      const collectionsAfterDeleted = streamedCollections?.filter((col) => col.order! > collectionToDelete.order!);
      if (collectionsAfterDeleted) {
        await batchUpdateCollection({
          items: collectionsAfterDeleted.map((col) => ({
            id: col.id,
            order: col.order! - 1,
          })),
        });
      }

      closeModal();
      removePanel(id);
    } catch (error) {
      console.error(error);
    }
  };

  const handleCancel = () => {
    closeModal();
  };

  return (
    <ConfirmationModal
      showModal={showModal}
      closeModal={closeModal}
      title="Delete Collection"
      message={`Are you sure you want to delete ${collection?.name} collection?`}
      onConfirm={handleSubmit}
      onCancel={handleCancel}
      loading={isDeleteCollectionLoading}
    />
  );
};
