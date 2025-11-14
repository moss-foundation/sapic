import { forwardRef } from "react";

import { cn } from "@/utils";

interface DragHandleButtonProps {
  className?: string;
  slim?: boolean;
  ghost?: boolean;
}

const DragHandleButton = forwardRef<HTMLDivElement, DragHandleButtonProps>(({ className, slim, ghost }, ref) => {
  return (
    <div
      ref={ref}
      className={cn(
        "flex cursor-grab items-center justify-center rounded",
        {
          "px-1 py-0.5": slim,
          "size-4": !slim,
          "": ghost,
          "background-(--moss-primary-background) shadow": !ghost,
        },
        className
      )}
    >
      <svg width="6" height="10" viewBox="0 0 6 10" fill="none" xmlns="http://www.w3.org/2000/svg">
        <path
          fillRule="evenodd"
          clipRule="evenodd"
          d="M0 0H2V2H0V0ZM4 0H6V2H4V0ZM2 4H0V6H2V4ZM4 4H6V6H4V4ZM2 8H0V10H2V8ZM4 8H6V10H4V8Z"
          fill="#525252"
        />
      </svg>
    </div>
  );
});

export { DragHandleButton };
