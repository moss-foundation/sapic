import SimpleBar, { type Props as SimpleBarProps } from "simplebar-react";

import { cn } from "@/utils";

export const Scrollbar = ({ children, className, ...props }: SimpleBarProps) => {
  return (
    <SimpleBar className={cn("h-full w-full", className)} {...props}>
      {children}
    </SimpleBar>
  );
};

export default Scrollbar;
