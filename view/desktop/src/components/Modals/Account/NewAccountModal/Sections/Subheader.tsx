import { ReactNode } from "react";

interface SubheaderProps {
  children: ReactNode;
}

export const Subheader = ({ children }: SubheaderProps) => {
  return (
    <div className="col-span-2 flex gap-2 pt-4 pb-3">
      {children}
      <div className="background-(--moss-border-color) my-auto h-px w-full" />
    </div>
  );
};
