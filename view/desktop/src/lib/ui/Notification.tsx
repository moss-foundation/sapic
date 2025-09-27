import { ComponentPropsWithoutRef, forwardRef, ReactNode } from "react";

import { cn } from "@/utils";
import { Primitive } from "@radix-ui/react-primitive";

import { Button } from "./Button";
import { Icon, type Icons } from "./Icon";

export interface NotificationProps extends ComponentPropsWithoutRef<typeof Primitive.div> {
  title?: string;
  description?: string;
  icon?: Icons | null;
  buttonText?: string;
  onButtonClick?: () => void;
  linkText?: string;
  onLinkClick?: () => void;
  children?: ReactNode;
  className?: string;
}

export const Notification = forwardRef<React.ElementRef<typeof Primitive.div>, NotificationProps>(
  (
    { title, description, icon, buttonText, onButtonClick, linkText, onLinkClick, children, className, ...props },
    ref
  ) => {
    if (children) {
      return (
        <Primitive.div
          ref={ref}
          className={cn(
            "relative flex w-90 items-start gap-2 rounded-lg border border-gray-700 p-4 shadow-lg",
            "background-[var(--moss-notification-bg)] text-[var(--moss-notification-text)]",
            className
          )}
          {...props}
        >
          {children}
        </Primitive.div>
      );
    }

    const displayIcon = icon !== null ? icon || "Info" : null;

    return (
      <Primitive.div
        ref={ref}
        className={cn(
          "relative flex w-90 items-start gap-2 rounded-lg border border-gray-700 px-2.5 pt-[7px] pb-4 shadow-lg",
          "background-[var(--moss-notification-bg)] text-[var(--moss-notification-text)]",
          className
        )}
        role="alert"
        {...props}
      >
        {displayIcon && <Icon icon={displayIcon} className="mt-0.5 size-4 flex-shrink-0" />}

        <div className="min-w-0 flex-1">
          {title && <div className="text-md leading-5 font-semibold text-[var(--moss-notification-text)]">{title}</div>}
          {description && (
            <div className="text-md pt-0.5 leading-4 text-[var(--moss-notification-text)]">{description}</div>
          )}

          {(buttonText || linkText) && (
            <div className="mt-3 flex items-center gap-3">
              {buttonText && onButtonClick && (
                <Button
                  onClick={onButtonClick}
                  className="hover:background-[var(--moss-notification-button-hover)] background-[var(--moss-notification-bg)] text-md h-auto rounded-md border border-[var(--moss-notification-button-outline)] px-3 py-[5px] text-[var(--moss-notification-text)] transition-colors"
                >
                  {buttonText}
                </Button>
              )}

              {linkText && onLinkClick && (
                <button
                  onClick={onLinkClick}
                  className="text-md cursor-pointer text-[var(--moss-notification-link-text)] underline-offset-4 transition-colors hover:underline"
                  type="button"
                >
                  {linkText}
                </button>
              )}
            </div>
          )}
        </div>
      </Primitive.div>
    );
  }
);

Notification.displayName = "Notification";

export default Notification;
