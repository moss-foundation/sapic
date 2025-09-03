import { HTMLAttributes } from "react";

import { cn } from "@/utils";

interface RootNodeLabelProps extends HTMLAttributes<HTMLDivElement> {
  label: string;
}

export const RootNodeLabel = ({ label, className, ...props }: RootNodeLabelProps) => {
  return (
    <div className={cn("w-full cursor-pointer truncate font-medium", className)} {...props}>
      {label}
    </div>
  );
};
