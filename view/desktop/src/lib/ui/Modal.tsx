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

  const handleDialogClick = (event: React.MouseEvent<HTMLDialogElement>) => {
    if (event.target === dialogRef.current) {
      onBackdropClick?.();
    }
  };

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
    >
      <dialog
        ref={dialogRef}
        className={cn(
          "mx-auto mt-[clamp(2vh,calc(18vh-50px),18vh)] flex max-h-[80vh] flex-col rounded-lg shadow-[0px_8px_40px_rgba(0,0,0,0.3)] transition-[display,opacity] transition-discrete duration-100 select-none backdrop:opacity-0 starting:opacity-0",
          className
        )}
        //MouseDown is used instead of Click to prevent the modal from closing when the user selects text inside the modal and than drags the cursor out of the dialog onto backdrop
        onMouseDown={handleDialogClick}
      >
        {children}
      </dialog>
    </div>,
    document.body
  );
};
