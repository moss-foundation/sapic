import { HTMLAttributes, ReactNode } from "react";

import { cn } from "@/utils";

interface ListChildrenProps extends HTMLAttributes<HTMLUListElement> {
  children: ReactNode;
  className?: string;
  show: boolean;
}
export const ListChildren = ({ children, className, show, ...props }: ListChildrenProps) => {
  if (!show) return null;

  return (
    <ul className={cn("relative h-full", className)} {...props}>
      {children}
    </ul>
  );
};
