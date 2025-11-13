import React from "react";

interface SectionTitleProps {
  children: React.ReactNode;
  className?: string;
}

export const SectionTitle = ({ children, className = "" }: SectionTitleProps) => {
  return (
    <div className="mb-2.5 flex w-[36rem] items-center gap-4">
      <h3 className={`font-medium ${className}`}>{children}</h3>
      <hr className="border-(--moss-border) flex-1" />
    </div>
  );
};
