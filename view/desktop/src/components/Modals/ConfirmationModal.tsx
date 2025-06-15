import React from "react";

import ButtonNeutralOutlined from "@/components/ButtonNeutralOutlined";
import ButtonPrimary from "@/components/ButtonPrimary";
import { Modal } from "@/lib/ui/Modal";

interface ConfirmationModalProps {
  showModal: boolean;
  closeModal: () => void;
  title: string;
  message: string;
  description?: string;
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
  description,
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
    <Modal
      showModal={showModal}
      onBackdropClick={handleCancel}
      className="background-(--moss-primary-background) h-52 w-[27rem] border border-(--moss-border-color) text-(--moss-primary-text)"
    >
      <div className="flex h-full flex-col">
        <div className="pt-2 pb-1.5">
          <h2 className="text-center text-base font-semibold text-(--moss-primary-text)">{title}</h2>
        </div>

        <div className="border-t border-(--moss-border-color)"></div>

        <div className="flex-1 px-5.5 py-4.5">
          <div className="mb-1 text-base font-medium">{message}</div>
          {description && (
            <div className="text-sm text-(--moss-secondary-text)">
              {description.split("\n").map((line, index) => (
                <p key={index} className={index > 0 ? "mt-2" : ""}>
                  {line}
                </p>
              ))}
            </div>
          )}
        </div>

        <div className="border-t border-(--moss-border-color)"></div>

        <div className="flex justify-end gap-3 px-6 py-4">
          <ButtonNeutralOutlined onClick={handleCancel}>{cancelLabel}</ButtonNeutralOutlined>
          <ButtonPrimary
            onClick={handleConfirm}
            className="!bg-(--moss-button-background-delete) !text-(--moss-button-text-delete) hover:!bg-(--moss-button-background-delete-hover)"
          >
            {confirmLabel}
          </ButtonPrimary>
        </div>
      </div>
    </Modal>
  );
};
