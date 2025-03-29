import { ComponentProps, forwardRef } from "react";

import { cn } from "@/utils";

export const ContentLayout = forwardRef<HTMLDivElement, ComponentProps<"div">>(
  ({ children, className, ...props }, ref) => (
    <div ref={ref} className={cn("bg-bgPrimary mb-5.5 flex-1 overflow-auto", className)} {...props}>
      {children}
    </div>
  )
);
