import { toast } from "sonner";

import { Button, Icon } from "@/lib/ui";

export interface NotificationContentProps {
  title: string;
  description?: string;
  icon?: "Info" | "Warning" | "Failed" | "Success";
  buttonText?: string;
  onButtonClick?: () => void;
  linkText?: string;
  onLinkClick?: () => void;
  onClose?: () => void;
}

export const createNotificationContent = ({
  title,
  description,
  icon = "Info",
  buttonText,
  onButtonClick,
  linkText,
  onLinkClick,
  onClose,
}: NotificationContentProps) => {
  return (
    <>
      {onClose && (
        <Button
          onClick={onClose}
          className="absolute top-2 z-10 cursor-pointer p-1 text-[var(--moss-notification-close-color)] opacity-70 transition-opacity hover:text-[var(--moss-notification-close-color)] hover:opacity-100"
          style={{ position: "absolute", top: "4px", right: "-18px" }}
        >
          <Icon icon="Close" className="size-4" />
        </Button>
      )}
      <div className="flex items-start gap-2.5 pr-8 text-base tracking-wide">
        <Icon icon={icon} className="mt-0.5 size-4 flex-shrink-0" />
        <div className="min-w-0 flex-1">
          <div className="leading-5 font-medium text-[var(--moss-notification-text)]">{title}</div>
          {description && <div className="pt-0.5 leading-4 text-[var(--moss-notification-text)]">{description}</div>}
          {(buttonText || linkText) && (
            <div className="mt-3 mb-1 flex items-center gap-3">
              {buttonText && (
                <Button
                  onClick={() => {
                    onButtonClick?.();
                  }}
                  className="hover:background-[var(--moss-notification-button-hover)] background-[var(--moss-notification-bg)] h-auto rounded-md border border-[var(--moss-notification-button-outline)] px-3 py-1 text-[var(--moss-notification-text)] transition-colors"
                >
                  {buttonText}
                </Button>
              )}
              {linkText && (
                <Button
                  onClick={() => {
                    onLinkClick?.();
                  }}
                  className="cursor-pointer text-[var(--moss-notification-link-text)] underline-offset-4 transition-colors hover:text-[var(--moss-notification-link-hover)]"
                >
                  {linkText}
                </Button>
              )}
            </div>
          )}
        </div>
      </div>
    </>
  );
};

export interface NotificationProps extends NotificationContentProps {
  duration?: number;
  persistent?: boolean;
}

export const showNotification = ({
  title,
  description,
  icon = "Info",
  buttonText,
  onButtonClick,
  linkText,
  onLinkClick,
  duration = 2000,
  persistent = false,
}: NotificationProps) => {
  const toastId = toast(
    createNotificationContent({
      title,
      description,
      icon,
      buttonText,
      onButtonClick: onButtonClick
        ? () => {
            onButtonClick();
            toast.dismiss(toastId);
          }
        : undefined,
      linkText,
      onLinkClick: onLinkClick
        ? () => {
            onLinkClick();
            toast.dismiss(toastId);
          }
        : undefined,
      onClose: () => toast.dismiss(toastId),
    }),
    { duration: persistent ? Infinity : duration }
  );

  return toastId;
};

export default { createNotificationContent, showNotification };
