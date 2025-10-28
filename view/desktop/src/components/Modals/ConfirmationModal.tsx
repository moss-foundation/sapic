import { useEffect, useState } from "react";

import { Button } from "@/lib/ui";

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
  onCancel?: () => void;
  variant?: "warning" | "danger" | "info";
  loading?: boolean;
}

export const ConfirmationModal = ({
  showModal,
  closeModal,
  title,
  message,
  description,
  confirmLabel = "OK",
  cancelLabel = "Cancel",
  onConfirm,
  onCancel,
  loading = false,
}: ConfirmationModalProps) => {
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
    onCancel?.();
    closeModal();
  };

  const handleBackdropClick = () => {
    if (!loading && allowBackdropClick) {
      handleCancel();
    }
  };

  return (
    <ModalForm
      showModal={showModal}
      onBackdropClick={handleBackdropClick}
      onSubmit={handleConfirm}
      className="background-(--moss-primary-background) border-(--moss-border) text-(--moss-primary-foreground) max-w-[400px] overflow-hidden border"
      title={title}
      content={
        <div className="flex h-full flex-col">
          <div className="py-4.5 flex-1">
            <div className="mb-1 text-base font-medium">{message}</div>
            {description && (
              <div className="text-(--moss-secondary-foreground) text-sm">
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
        <div className="py-0.75 flex items-center justify-end gap-3">
          <Button intent="outlined" onClick={handleCancel}>
            {cancelLabel}
          </Button>
          <Button intent="danger" type="submit">
            {confirmLabel}
          </Button>
        </div>
      }
    />
  );
};
