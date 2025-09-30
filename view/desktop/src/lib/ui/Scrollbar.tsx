import SimpleBar, { type Props as SimpleBarProps } from "simplebar-react";

import { cn } from "@/utils";

export const Scrollbar = ({ children, className, classNames, ...props }: SimpleBarProps) => {
  return <div className={cn("h-full w-full overflow-auto", className)}>{children}</div>;
  return (
    <SimpleBar
      classNames={{
        contentEl: "w-full",
        ...classNames,
      }}
      className={cn("h-full w-full", className)}
      {...props}
    >
      {children}
    </SimpleBar>
  );
};

export default Scrollbar;
