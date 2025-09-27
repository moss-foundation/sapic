import { ComponentPropsWithoutRef, forwardRef, ReactNode } from "react";
import { cva, type VariantProps } from "class-variance-authority";

import { cn } from "@/utils";
import { Primitive } from "@radix-ui/react-primitive";

import { Button } from "./Button";
import { Icon, type Icons } from "./Icon";

const notificationVariants = cva("relative flex items-start gap-3 rounded-lg border p-4 shadow-lg backdrop-blur-sm", {
  variants: {
    variant: {
      info: "border-blue-800/50 bg-blue-950/90 text-blue-50",
      warning: "border-yellow-800/50 bg-yellow-950/90 text-yellow-50",
      error: "border-red-800/50 bg-red-950/90 text-red-50",
    },
  },
  defaultVariants: {
    variant: "info",
  },
});

const iconVariants = cva("mt-0.5 flex-shrink-0", {
  variants: {
    variant: {
      info: "text-blue-400",
      warning: "text-yellow-400",
      error: "text-red-400",
    },
  },
  defaultVariants: {
    variant: "info",
  },
});

const titleVariants = cva("text-sm leading-5 font-semibold", {
  variants: {
    variant: {
      info: "text-blue-100",
      warning: "text-yellow-100",
      error: "text-red-100",
    },
  },
  defaultVariants: {
    variant: "info",
  },
});

const descriptionVariants = cva("mt-1 text-sm leading-5", {
  variants: {
    variant: {
      info: "text-blue-200",
      warning: "text-yellow-200",
      error: "text-red-200",
    },
  },
  defaultVariants: {
    variant: "info",
  },
});

const linkVariants = cva("cursor-pointer text-sm underline-offset-4 transition-colors hover:underline", {
  variants: {
    variant: {
      info: "text-blue-300 hover:text-blue-200",
      warning: "text-yellow-300 hover:text-yellow-200",
      error: "text-red-300 hover:text-red-200",
    },
  },
  defaultVariants: {
    variant: "info",
  },
});

const buttonVariants = cva("text-sm font-medium transition-colors", {
  variants: {
    variant: {
      info: "border-blue-500 bg-blue-600 text-white hover:bg-blue-700",
      warning: "border-yellow-500 bg-yellow-600 text-white hover:bg-yellow-700",
      error: "border-red-500 bg-red-600 text-white hover:bg-red-700",
    },
  },
  defaultVariants: {
    variant: "info",
  },
});

type NotificationVariant = "info" | "warning" | "error";

const iconMap: Record<NotificationVariant, Icons> = {
  info: "Info",
  warning: "Warning",
  error: "Error",
};

export interface NotificationProps
  extends ComponentPropsWithoutRef<typeof Primitive.div>,
    VariantProps<typeof notificationVariants> {
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
    {
      variant = "info",
      title,
      description,
      icon,
      buttonText,
      onButtonClick,
      linkText,
      onLinkClick,
      children,
      className,
      ...props
    },
    ref
  ) => {
    if (children) {
      return (
        <Primitive.div ref={ref} className={cn(notificationVariants({ variant }), className)} {...props}>
          {children}
        </Primitive.div>
      );
    }

    const displayIcon = icon !== null ? icon || iconMap[variant!] : null;

    return (
      <Primitive.div ref={ref} className={cn(notificationVariants({ variant }), className)} role="alert" {...props}>
        {displayIcon && <Icon icon={displayIcon} className={cn("size-5", iconVariants({ variant }))} />}

        <div className="min-w-0 flex-1">
          {title && <div className={cn(titleVariants({ variant }))}>{title}</div>}
          {description && <div className={cn(descriptionVariants({ variant }))}>{description}</div>}

          {(buttonText || linkText) && (
            <div className="mt-3 flex items-center gap-3">
              {buttonText && onButtonClick && (
                <Button
                  onClick={onButtonClick}
                  className={cn(
                    "h-auto rounded-md border px-3 py-1.5 text-sm font-medium",
                    buttonVariants({ variant })
                  )}
                >
                  {buttonText}
                </Button>
              )}

              {linkText && onLinkClick && (
                <button onClick={onLinkClick} className={cn(linkVariants({ variant }))} type="button">
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
