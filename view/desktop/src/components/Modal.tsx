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
  const handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    onSubmit?.(e);
  };

  return createPortal(
    <div
      className={cn(
        "fixed top-0 left-0 z-[9999] h-full w-full items-center justify-center transition-[display,opacity] transition-discrete duration-100 starting:opacity-0",
        {
          "bg-black/70": backdropFilter === "darken",
          "backdrop-blur": backdropFilter === "blur",
          "hidden opacity-0": !showModal,
          "flex opacity-100": showModal,
        }
      )}
      onClick={onBackdropClick}
    >
      <form
        onSubmit={handleSubmit}
        onClick={(e) => e.stopPropagation()}
        className="background-(--moss-primary-background) flex max-w-[544px] min-w-64 flex-col rounded-lg shadow-2xl"
      >
        {title && (
          <h2 className="flex items-center justify-center border-b border-(--moss-border-color) py-2 font-semibold">
            {title}
          </h2>
        )}
        <div className="p-6">{content}</div>
        <div className="border-t border-(--moss-border-color) px-6 py-4">{footer}</div>
      </form>
    </div>,
    document.body
  );
};
