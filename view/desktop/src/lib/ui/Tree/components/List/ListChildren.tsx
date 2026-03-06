import { HTMLAttributes, ReactNode } from "react";

import { cn } from "@/utils";

interface ListChildrenProps extends HTMLAttributes<HTMLUListElement> {
  children: ReactNode;
  className?: string;
}
export const ListChildren = ({ children, className, ...props }: ListChildrenProps) => {
  return (
    <ul className={cn("relative h-full", className)} {...props}>
      {children}
    </ul>
  );
};
