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
            "relative flex w-90 items-start gap-3 rounded-lg border border-gray-700 p-4 shadow-lg",
            "bg-[var(--moss-notification-bg)] text-[var(--moss-notification-text)]",
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
          "relative flex w-90 items-start gap-3 rounded-lg border border-gray-700 p-4 shadow-lg",
          "bg-[var(--moss-notification-bg)] text-[var(--moss-notification-text)]",
          className
        )}
        role="alert"
        {...props}
      >
        {displayIcon && <Icon icon={displayIcon} className="mt-0.5 size-5 flex-shrink-0 text-orange-400" />}

        <div className="min-w-0 flex-1">
          {title && <div className="text-sm leading-5 font-semibold text-[var(--moss-notification-text)]">{title}</div>}
          {description && (
            <div className="mt-1 text-sm leading-5 text-[var(--moss-notification-text)]">{description}</div>
          )}

          {(buttonText || linkText) && (
            <div className="mt-3 flex items-center gap-3">
              {buttonText && onButtonClick && (
                <Button
                  onClick={onButtonClick}
                  className="h-auto rounded-md border px-3 py-1.5 text-sm font-medium transition-colors hover:bg-[var(--moss-notification-button-hover)]"
                  style={{
                    borderColor: "var(--moss-notification-button-outline)",
                    backgroundColor: "var(--moss-notification-bg)",
                    color: "var(--moss-notification-text)",
                  }}
                >
                  {buttonText}
                </Button>
              )}

              {linkText && onLinkClick && (
                <button
                  onClick={onLinkClick}
                  className="cursor-pointer text-sm underline-offset-4 transition-colors hover:underline"
                  style={{ color: "var(--moss-notification-link-text)" }}
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
