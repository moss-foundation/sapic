import { useState } from "react";

import ButtonNeutralOutlined from "@/components/ButtonNeutralOutlined";
import ButtonPrimary from "@/components/ButtonPrimary";
import InputOutlined from "@/components/InputOutlined";
import { ModalForm } from "@/components/ModalForm";
import { useDeleteCollection } from "@/hooks/collection/useDeleteCollection";

import { ModalWrapperProps } from "../types";

export const DeleteCollectionModal = ({ closeModal, showModal }: ModalWrapperProps) => {
  const { mutateAsync: deleteCollection, isPending } = useDeleteCollection();

  const [id, setId] = useState("");

  const handleSubmit = async () => {
    await deleteCollection({
      id,
    });
  };

  const handleCancel = () => {
    closeModal();
    resetForm();
  };

  const resetForm = () => {
    setTimeout(() => {
      setId("");
    }, 200);
  };

  const isSubmitDisabled = id.length === 0 || isPending;

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
        <div className="flex flex-col gap-2">
          <div className="grid grid-cols-[min-content_1fr] grid-rows-[repeat(2,1fr)] items-center gap-x-3.75">
            <div className="self-start">Id:</div>
            <InputOutlined
              value={id}
              className="max-w-72"
              onChange={(e) => setId(e.target.value)}
              pattern="^[^/:\\*?|]+$"
              required
            />
            <p className="col-start-2 max-w-72 text-xs text-(--moss-secondary-text)">{`Invalid filename characters (e.g. / \ : * ? " < > |) will be escaped`}</p>
          </div>
        </div>
      }
      footer={
        <div className="flex items-center justify-end py-0.75">
          <div className="flex gap-3 px-0.25 py-1.25">
            <ButtonNeutralOutlined onClick={handleCancel}>Cancel</ButtonNeutralOutlined>
            <ButtonPrimary disabled={isSubmitDisabled} type="submit">
              Delete
            </ButtonPrimary>
          </div>
        </div>
      }
    />
  );
};
