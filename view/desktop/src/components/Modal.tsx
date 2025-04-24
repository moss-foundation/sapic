import { useEffect, useRef } from "react";
import { createPortal } from "react-dom";

import { cn } from "@/utils";

interface ModalProps {
  showModal: boolean;
  title?: string;
  content?: React.ReactNode;
  footer?: React.ReactNode;
  backdropFilter?: "blur" | "darken" | "none";
  notAForm?: boolean;
  onSubmit?: (e: React.FormEvent<HTMLFormElement>) => void;
  onBackdropClick?: () => void;
}

export const Modal = ({
  backdropFilter = "blur",
  showModal,
  title,
  content,
  footer,
  onSubmit,
  onBackdropClick,
}: ModalProps) => {
  const dialogRef = useRef<HTMLDialogElement>(null);

  const handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    onSubmit?.(e);
  };

  useEffect(() => {
    if (showModal) {
      dialogRef.current?.showModal();
    } else {
      dialogRef.current?.close();
    }
  }, [showModal]);

  return createPortal(
    <div
      className={cn(
        "fixed top-0 left-0 z-[9999] h-full w-full transition-[display,opacity] transition-discrete duration-100 starting:opacity-0",
        {
          "bg-black/70": backdropFilter === "darken",
          "backdrop-blur": backdropFilter === "blur",
          "hidden opacity-0": !showModal,
          "block opacity-100": showModal,
        }
      )}
      style={{
        "WebkitBackdropFilter": "blur(8px)", // BackdropFilter doesn't work on Linux without this
      }}
      onClick={onBackdropClick}
    >
      <dialog
        ref={dialogRef}
        className={cn(
          "background-(--moss-primary-background) mx-auto mt-[9%] flex max-w-[544px] min-w-64 flex-col rounded-lg shadow-[0px_8px_40px_rgba(0,0,0,0.3)] transition-[display,opacity] transition-discrete duration-100 backdrop:opacity-0 starting:opacity-0"
        )}
      >
        <form
          onSubmit={handleSubmit}
          onClick={(e) => e.stopPropagation()}
          key={showModal ? "modal-open" : "modal-closed"}
        >
          {title && (
            <h2 className="flex items-center justify-center border-b border-(--moss-border-color) py-1.5 font-semibold">
              {title}
            </h2>
          )}
          <div className="px-6 pt-3 pb-5">{content}</div>
          <div className="border-t border-(--moss-border-color) px-6 py-2">{footer}</div>
        </form>
      </dialog>
    </div>,
    document.body
  );
};
