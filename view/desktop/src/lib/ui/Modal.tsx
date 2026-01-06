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
    const dialog = dialogRef.current;

    if (showModal) {
      dialog?.showModal();
    } else {
      dialog?.close();
    }
  }, [showModal]);

  const handleDialogClick = (event: React.MouseEvent<HTMLDialogElement>) => {
    // When using native dialog, clicking the ::backdrop pseudo-element
    // registers as a click on the dialog element itself.
    if (event.target === dialogRef.current) {
      onBackdropClick?.();
    }
  };

  return createPortal(
    <dialog
      ref={dialogRef}
      onMouseDown={handleDialogClick}
      className={cn(
        // BASE STYLES
        "fixed m-auto",
        "max-h-[80vh] min-h-0 w-full max-w-lg", // Added max-w-lg for standard modal width (adjust as needed)
        "flex flex-col overflow-hidden rounded-lg",
        "shadow-[0px_8px_40px_rgba(0,0,0,0.3)]",
        "bg-white",

        // BACKDROP STYLING (Native dialog's ::backdrop)
        {
          "backdrop:bg-black/70": backdropFilter === "darken",
          "backdrop:backdrop-blur-md": backdropFilter === "blur",
          "backdrop:bg-transparent": backdropFilter === "none",
        },

        // ANIMATION (entry/exit)
        "allow-discrete transition-[opacity,transform,display] duration-200",
        "backdrop:allow-discrete backdrop:transition-[opacity,display] backdrop:duration-200",

        // STARTING STATE (Before open)
        "opacity-0 backdrop:opacity-0",

        // OPEN STATE (Target state)
        "open:opacity-100 open:backdrop:opacity-100",

        className
      )}
    >
      {children}
    </dialog>,
    document.body
  );
};
