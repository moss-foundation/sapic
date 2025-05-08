import { Modal, ModalProps } from "@/lib/ui";
import { cn } from "@/utils";

interface ModalFormProps extends ModalProps {
  title?: string;
  content?: React.ReactNode;
  footer?: React.ReactNode;
  onSubmit?: (e: React.FormEvent<HTMLFormElement>) => void;
  titleClassName?: string;
  footerClassName?: string;
}

export const ModalForm = ({
  title,
  content,
  footer,
  onSubmit,
  titleClassName,
  footerClassName,
  ...props
}: ModalFormProps) => {
  const handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    onSubmit?.(e);
  };

  return (
    <Modal {...props}>
      <form onSubmit={handleSubmit} onClick={(e) => e.stopPropagation()}>
        {title && (
          <h2 className={cn("flex items-center justify-center py-1.5 font-medium", titleClassName)}>{title}</h2>
        )}
        <div className="px-6 pt-3 pb-5">{content}</div>
        <div className={cn("px-6 py-2", footerClassName)}>{footer}</div>
      </form>
    </Modal>
  );
};
