import { useEffect, useRef } from "react";
import { createPortal } from "react-dom";

import { cn } from "@/utils";

export interface ModalProps {
  showModal: boolean;
  backdropFilter?: "blur" | "darken" | "none";
  onBackdropClick?: () => void;
  className?: string;
  children?: React.ReactNode;
}

export const Modal = ({ backdropFilter = "blur", showModal, onBackdropClick, className, children }: ModalProps) => {
  const dialogRef = useRef<HTMLDialogElement>(null);

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
      // onClick={onBackdropClick}
      onMouseDown={onBackdropClick}
    >
      <dialog
        ref={dialogRef}
        className={cn(
          "mx-auto mt-[9%] flex max-w-[544px] min-w-64 flex-col rounded-lg shadow-[0px_8px_40px_rgba(0,0,0,0.3)] transition-[display,opacity] transition-discrete duration-100 select-none backdrop:opacity-0 starting:opacity-0",
          className
        )}
      >
        {children}
      </dialog>
    </div>,
    document.body
  );
};
