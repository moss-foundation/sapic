import React from "react";

import ButtonNeutralOutlined from "@/components/ButtonNeutralOutlined";
import ButtonPrimary from "@/components/ButtonPrimary";
import { ModalForm } from "@/components/ModalForm";

interface ConfirmationModalProps {
  showModal: boolean;
  closeModal: () => void;
  title: string;
  message: string;
  confirmLabel?: string;
  cancelLabel?: string;
  onConfirm: () => void;
  variant?: "warning" | "danger" | "info";
}

export const ConfirmationModal: React.FC<ConfirmationModalProps> = ({
  showModal,
  closeModal,
  title,
  message,
  confirmLabel = "OK",
  cancelLabel = "Cancel",
  onConfirm,
}) => {
  const handleConfirm = () => {
    onConfirm();
    closeModal();
  };

  const handleCancel = () => {
    closeModal();
  };

  return (
    <ModalForm
      showModal={showModal}
      onBackdropClick={handleCancel}
      className="background-(--moss-primary-background) h-52 w-[27rem] border border-(--moss-border-color)"
      content={
        <div>
          <div className="px-6 pt-6 pb-4">
            <h2 className="mb-4 text-center text-lg font-semibold text-(--moss-primary-text)">{title}</h2>
            <div className="mb-3 text-sm text-(--moss-primary-text)">
              {message.split("\n").map((line, index) => (
                <p key={index} className={index > 0 ? "mt-3" : ""}>
                  {line}
                </p>
              ))}
            </div>
          </div>
        </div>
      }
      footer={
        <div className="flex justify-end gap-3 px-6 pb-6">
          <ButtonNeutralOutlined onClick={handleCancel}>{cancelLabel}</ButtonNeutralOutlined>
          <ButtonPrimary
            onClick={handleConfirm}
            className="!bg-(--moss-button-background-delete) !text-(--moss-button-text-delete) hover:!bg-(--moss-button-background-delete-hover)"
          >
            {confirmLabel}
          </ButtonPrimary>
        </div>
      }
      titleClassName=""
      footerClassName=""
    />
  );
};
