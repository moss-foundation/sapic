import { useDeleteCollection, useStreamedCollections } from "@/hooks";
import { useTabbedPaneStore } from "@/store/tabbedPane";

import { ConfirmationModal } from "../ConfirmationModal";
import { ModalWrapperProps } from "../types";

export const DeleteCollectionModal = ({ closeModal, showModal, id }: ModalWrapperProps & { id: string }) => {
  const { data: streamedCollections } = useStreamedCollections();
  const { mutateAsync: deleteCollection, isPending: isDeleteCollectionLoading } = useDeleteCollection();

  const { removePanel } = useTabbedPaneStore();

  const collection = streamedCollections?.find((collection) => collection.id === id);

  const handleSubmit = async () => {
    try {
      await deleteCollection({ id });
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
