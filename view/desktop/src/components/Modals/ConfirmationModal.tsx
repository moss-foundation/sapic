import React from "react";

import ButtonNeutralOutlined from "@/components/ButtonNeutralOutlined";
import ButtonPrimary from "@/components/ButtonPrimary";
import { ModalForm } from "@/components/ModalForm";
import { Icon, Icons } from "@/lib/ui";

interface ConfirmationModalProps {
  showModal: boolean;
  closeModal: () => void;
  title: string;
  message: string;
  confirmLabel?: string;
  cancelLabel?: string;
  onConfirm: () => void;
  icon?: Icons;
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
  icon,
  variant = "warning",
}) => {
  const handleConfirm = () => {
    onConfirm();
    closeModal();
  };

  const handleCancel = () => {
    closeModal();
  };

  const getIconForVariant = (): Icons => {
    if (icon) return icon;

    switch (variant) {
      case "danger":
        return "Error";
      case "warning":
        return "Warning";
      case "info":
        return "Info";
      default:
        return "Warning";
    }
  };

  const getIconColorForVariant = () => {
    switch (variant) {
      case "danger":
        return "text-red-500";
      case "warning":
        return "text-yellow-500";
      case "info":
        return "text-blue-500";
      default:
        return "text-yellow-500";
    }
  };

  return (
    <ModalForm
      showModal={showModal}
      onBackdropClick={handleCancel}
      className="background-(--moss-primary-background) max-w-md min-w-96 border border-(--moss-border-color)"
      content={
        <div>
          <div className="flex items-center gap-3 px-6 pt-5 pb-2">
            <Icon icon={getIconForVariant()} className={`size-6 ${getIconColorForVariant()}`} />
            <span className="text-lg font-medium text-(--moss-primary-text)">{title}</span>
          </div>
          <div className="px-6 py-2">
            <p className="text-sm leading-relaxed text-(--moss-secondary-text)">{message}</p>
          </div>
        </div>
      }
      footer={
        <div className="flex justify-end gap-3 border-t border-(--moss-border-color) px-6 py-4">
          <ButtonNeutralOutlined onClick={handleCancel}>{cancelLabel}</ButtonNeutralOutlined>
          <ButtonPrimary onClick={handleConfirm}>{confirmLabel}</ButtonPrimary>
        </div>
      }
      titleClassName=""
      footerClassName=""
    />
  );
};
