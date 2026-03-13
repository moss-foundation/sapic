import { HTMLAttributes } from "react";

import { cn } from "@/utils";

interface RootLabelProps extends HTMLAttributes<HTMLDivElement> {
  label: string;
}

export const RootLabel = ({ label, className, ...props }: RootLabelProps) => {
  return (
    <div className={cn("w-full cursor-pointer truncate font-medium", className)} {...props}>
      {label}
    </div>
  );
};
