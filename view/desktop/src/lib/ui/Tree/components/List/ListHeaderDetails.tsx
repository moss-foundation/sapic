import { HTMLAttributes } from "react";

import { cn } from "@/utils";

export const ListHeaderDetails = ({ className, children, ...props }: HTMLAttributes<HTMLSpanElement>) => {
  return (
    <span className={cn("flex w-full min-w-0 items-center gap-1.5", className)} {...props}>
      {children}
    </span>
  );
};
