import { toast } from "sonner";

import { Button, Icon, Link } from "@/lib/ui";

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
        <button
          onClick={onClose}
          className="text-(--moss-notification-close) hover:text-(--moss-notification-close) absolute right-[8px] top-[10px] z-10 cursor-pointer p-1 font-sans text-base font-normal opacity-70 transition-opacity hover:opacity-100"
        >
          <Icon icon="Close" />
        </button>
      )}
      <div className="-ml-1.5 -mt-0.5 flex items-start gap-2 pr-4 font-sans text-base font-normal">
        <Icon icon={icon} className="mt-0.5 size-4 flex-shrink-0" />
        <div className="min-w-0 flex-1">
          <div className="text-(--moss-notification-foreground) font-medium leading-5">{title}</div>
          {description && (
            <div
              className={`text-(--moss-notification-foreground) pt-0.5 leading-4 ${!(buttonText || linkText) ? "mb-1" : ""}`}
            >
              {description}
            </div>
          )}
          {(buttonText || linkText) && (
            <div className="mb-1 mt-3 flex items-center gap-3">
              {buttonText && (
                <Button
                  intent="outlined"
                  onClick={() => {
                    onButtonClick?.();
                  }}
                >
                  {buttonText}
                </Button>
              )}
              {linkText && (
                <Link
                  onClick={() => {
                    onLinkClick?.();
                  }}
                >
                  {linkText}
                </Link>
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
