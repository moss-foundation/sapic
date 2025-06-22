import React from "react";
import { cn } from "@/utils";

interface PageContainerHeaderProps {
  children?: React.ReactNode;
  toolbar?: React.ReactNode;
  className?: string;
}

export const PageContainerHeader: React.FC<PageContainerHeaderProps> = ({ children, toolbar, className }) => {
  return (
    <header className={cn("background-(--moss-primary-background) relative h-9", "flex-shrink-0", className)}>
      {/* Main header content */}
      {children}

      {/* Toolbar positioned absolutely on the right */}
      {toolbar && <div className="absolute top-1/2 right-3 flex -translate-y-1/2 items-center">{toolbar}</div>}
    </header>
  );
};
