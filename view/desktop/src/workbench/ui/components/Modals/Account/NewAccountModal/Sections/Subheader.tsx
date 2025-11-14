import { ReactNode } from "react";

interface SubheaderProps {
  children: ReactNode;
}

export const Subheader = ({ children }: SubheaderProps) => {
  return <div className="flex justify-between gap-2 py-1.5">{children}</div>;
};
