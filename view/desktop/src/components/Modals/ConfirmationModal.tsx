import React from "react";

import ButtonNeutralOutlined from "@/components/ButtonNeutralOutlined";

import ButtonDanger from "../ButtonDanger";
import { ModalForm } from "../ModalForm";

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
    <ModalForm
      onSubmit={handleConfirm}
      showModal={showModal}
      onBackdropClick={handleCancel}
      className="background-(--moss-primary-background) max-w-[27rem]"
      titleClassName="border-b border-(--moss-border-color)"
      footerClassName="border-t border-(--moss-border-color)"
      title={title}
      content={
        <div className="flex h-full flex-col">
          <div className="flex-1 py-4.5">
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
        </div>
      }
      footer={
        <div className="flex items-center justify-end gap-3 py-0.75">
          <ButtonNeutralOutlined onClick={handleCancel}>{cancelLabel}</ButtonNeutralOutlined>
          <ButtonDanger type="submit">{confirmLabel}</ButtonDanger>
        </div>
      }
    />
  );
};
