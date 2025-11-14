import { FormEvent, ReactNode } from "react";

import { Modal, ModalProps } from "@/lib/ui";
import { cn } from "@/utils";

interface ModalFormProps extends ModalProps {
  title?: string;
  content?: ReactNode;
  footer?: ReactNode;
  onSubmit?: (e: FormEvent<HTMLFormElement>) => void;
  onBackdropClick?: () => void;
  titleClassName?: string;
  footerClassName?: string;
  className?: string;
}

export const ModalForm = ({
  title,
  content,
  footer,
  onSubmit,
  titleClassName,
  footerClassName,
  className,
  ...props
}: ModalFormProps) => {
  const handleSubmit = (e: FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    onSubmit?.(e);
  };

  return (
    <Modal className={className} {...props}>
      <form onSubmit={handleSubmit} onClick={(e) => e.stopPropagation()} onMouseDown={(e) => e.stopPropagation()}>
        {title && (
          <h2
            className={cn(
              "border-(--moss-border) flex items-center justify-center border-b py-1.5 font-medium",
              titleClassName
            )}
          >
            {title}
          </h2>
        )}
        <div className="px-6 pb-5 pt-3">{content}</div>
        <div className={cn("border-(--moss-border) border-t px-6 py-2", footerClassName)}>{footer}</div>
      </form>
    </Modal>
  );
};
