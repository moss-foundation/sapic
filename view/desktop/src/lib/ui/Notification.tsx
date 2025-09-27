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
            "relative flex items-start gap-3 rounded-lg border border-gray-700 bg-gray-800 p-4 text-white shadow-lg",
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
          "relative flex items-start gap-3 rounded-lg border border-gray-700 bg-gray-800 p-4 text-white shadow-lg",
          className
        )}
        role="alert"
        {...props}
      >
        {displayIcon && <Icon icon={displayIcon} className="mt-0.5 size-5 flex-shrink-0 text-orange-400" />}

        <div className="min-w-0 flex-1">
          {title && <div className="text-sm leading-5 font-semibold text-white">{title}</div>}
          {description && <div className="mt-1 text-sm leading-5 text-gray-300">{description}</div>}

          {(buttonText || linkText) && (
            <div className="mt-3 flex items-center gap-3">
              {buttonText && onButtonClick && (
                <Button
                  onClick={onButtonClick}
                  className="h-auto rounded-md border border-gray-600 bg-gray-700 px-3 py-1.5 text-sm font-medium text-white transition-colors hover:bg-gray-600"
                >
                  {buttonText}
                </Button>
              )}

              {linkText && onLinkClick && (
                <button
                  onClick={onLinkClick}
                  className="cursor-pointer text-sm text-gray-400 underline-offset-4 transition-colors hover:text-gray-300 hover:underline"
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
