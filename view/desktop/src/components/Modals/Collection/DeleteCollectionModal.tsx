import ButtonNeutralOutlined from "@/components/ButtonNeutralOutlined";
import ButtonPrimary from "@/components/ButtonPrimary";
import { ModalForm } from "@/components/ModalForm";
import { useCollectionsStore } from "@/store/collections";

import { ModalWrapperProps } from "../types";

export const DeleteCollectionModal = ({
  closeModal,
  showModal,
  collectionId,
}: ModalWrapperProps & { collectionId: string }) => {
  const { deleteCollection, isDeleteCollectionLoading, streamedCollections } = useCollectionsStore();
  const collection = streamedCollections.find((collection) => collection.id === collectionId);

  const handleSubmit = async () => {
    await deleteCollection(collectionId);
    closeModal();
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
            <ButtonPrimary disabled={isSubmitDisabled} type="submit">
              Delete
            </ButtonPrimary>
          </div>
        </div>
      }
    />
  );
};
