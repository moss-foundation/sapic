import ButtonDanger from "@/components/ButtonDanger";
import ButtonNeutralOutlined from "@/components/ButtonNeutralOutlined";
import { ModalForm } from "@/components/ModalForm";
import { useDeleteCollection } from "@/hooks";
import { useCollectionsStore } from "@/store/collections";
import { useTabbedPaneStore } from "@/store/tabbedPane";

import { ModalWrapperProps } from "../types";

export const DeleteCollectionModal = ({
  closeModal,
  showModal,
  collectionId,
}: ModalWrapperProps & { collectionId: string }) => {
  const { streamedCollections } = useCollectionsStore();
  const { mutateAsync: deleteCollection, isPending: isDeleteCollectionLoading } = useDeleteCollection();

  const { removePanel } = useTabbedPaneStore();

  const collection = streamedCollections.find((collection) => collection.id === collectionId);

  const handleSubmit = async () => {
    try {
      await deleteCollection(collectionId);
      closeModal();
      removePanel(collectionId);
    } catch (error) {
      console.error(error);
    }
  };

  const handleCancel = () => {
    closeModal();
  };
  const isSubmitDisabled = isDeleteCollectionLoading;

  return (
    <ModalForm
      title="Delete Collection"
      onBackdropClick={handleCancel}
      showModal={showModal}
      onSubmit={handleSubmit}
      className="background-(--moss-primary-background)"
      titleClassName="border-b border-(--moss-border-color)"
      footerClassName="border-t border-(--moss-border-color)"
      content={
        <div>
          You sure you want to delete <span className="font-bold">{collection?.name}</span>?
        </div>
      }
      footer={
        <div className="flex items-center justify-end py-0.75">
          <div className="flex gap-3 px-0.25 py-1.25">
            <ButtonNeutralOutlined onClick={handleCancel} type="button">
              Cancel
            </ButtonNeutralOutlined>
            <ButtonDanger disabled={isSubmitDisabled} type="submit">
              Delete
            </ButtonDanger>
          </div>
        </div>
      }
    />
  );
};
