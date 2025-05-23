import { ComponentPropsWithoutRef } from "react";

import { cn } from "@/utils/cn";

export const Separator = ({ className, ...props }: ComponentPropsWithoutRef<"div">) => {
  return <div className={cn("my-1 h-px w-full", className)} {...props} />;
};
