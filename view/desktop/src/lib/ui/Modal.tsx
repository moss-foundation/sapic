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
      key={showModal ? "show" : "hidden"}
      className={cn(
        "transition-discrete starting:opacity-0 fixed left-0 top-0 z-[9999] h-full w-full transition-[display,opacity] duration-100",
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
          "transition-discrete starting:opacity-0 mx-auto mt-[clamp(2vh,calc(18vh-50px),18vh)] flex max-h-[80vh] min-h-0 select-none flex-col overflow-hidden rounded-lg shadow-[0px_8px_40px_rgba(0,0,0,0.3)] transition-[display,opacity] duration-100 backdrop:opacity-0",
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
