import React from "react";
import { cn } from "@/utils";

interface PageContainerHeaderProps {
  children?: React.ReactNode;
  className?: string;
}

export const PageContainerHeader: React.FC<PageContainerHeaderProps> = ({ children, className }) => {
  return (
    <header className={cn("background-(--moss-primary-background) relative h-9", "flex-shrink-0", className)}>
      {/* Main header content */}
      {children}
    </header>
  );
};
