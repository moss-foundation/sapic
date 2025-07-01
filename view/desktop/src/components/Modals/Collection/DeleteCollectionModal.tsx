import ButtonDanger from "@/components/ButtonDanger";
import ButtonNeutralOutlined from "@/components/ButtonNeutralOutlined";
import { ModalForm } from "@/components/ModalForm";
import { useDeleteCollection, useStreamedCollections } from "@/hooks";
import { useTabbedPaneStore } from "@/store/tabbedPane";

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
  const isSubmitDisabled = isDeleteCollectionLoading;

  return (
    <ModalForm
      size="small"
      title="Delete"
      onBackdropClick={handleCancel}
      showModal={showModal}
      onSubmit={handleSubmit}
      className="background-(--moss-primary-background)"
      titleClassName="border-b border-(--moss-border-color)"
      footerClassName="border-t border-(--moss-border-color)"
      content={
        <div className="pt-2 pb-1.5">
          <div className="flex-1 gap-2">
            <div className="mb-1 text-base font-medium">{`Delete "${collection?.name}"?`}</div>
            <div className="text-sm text-(--moss-secondary-text)">
              This will delete all requests, endpoints, and other items in this collection. This action cannot be
              undone.
            </div>
          </div>
        </div>
      }
      footer={
        <div className="flex items-center justify-end py-0.75">
          <div className="flex gap-3 px-0.25 py-1.25">
            <ButtonNeutralOutlined onClick={handleCancel} type="button">
              Close
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
