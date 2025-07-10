import React, { useState, useEffect } from "react";

import ButtonNeutralOutlined from "@/components/ButtonNeutralOutlined";
import { Modal } from "@/lib/ui/Modal";

import ButtonDanger from "../ButtonDanger";

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
  loading?: boolean;
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
  loading = false,
}) => {
  const [allowBackdropClick, setAllowBackdropClick] = useState(false);

  useEffect(() => {
    if (showModal) {
      const timer = setTimeout(() => {
        setAllowBackdropClick(true);
      }, 100);

      return () => clearTimeout(timer);
    }

    setAllowBackdropClick(false);
    return undefined;
  }, [showModal]);

  const handleConfirm = () => {
    if (loading) return;
    onConfirm();
  };

  const handleCancel = () => {
    if (loading) return;
    closeModal();
  };

  const handleBackdropClick = () => {
    if (!loading && allowBackdropClick) {
      handleCancel();
    }
  };

  return (
    <Modal
      showModal={showModal}
      onBackdropClick={loading ? undefined : handleBackdropClick}
      className="background-(--moss-primary-background) h-52 w-[27rem] overflow-hidden border border-(--moss-border-color) text-(--moss-primary-text)"
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
          <ButtonNeutralOutlined onClick={handleCancel} disabled={loading}>
            {cancelLabel}
          </ButtonNeutralOutlined>
          <ButtonDanger onClick={handleConfirm} disabled={loading}>
            {confirmLabel}
          </ButtonDanger>
        </div>
      </div>
    </Modal>
  );
};
