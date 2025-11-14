import { ReactNode } from "react";

interface SectionProps {
  title?: string;
  children: ReactNode;
}

export const Section = ({ title, children }: SectionProps) => {
  return (
    <div className="flex flex-col gap-2">
      {title && <h3 className="text-lg font-semibold">{title}</h3>}
      <div>{children}</div>
    </div>
  );
};
