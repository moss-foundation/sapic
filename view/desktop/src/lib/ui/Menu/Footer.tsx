import { forwardRef } from "react";

import { Menu } from "@/lib/ui";
import { cn } from "@/utils";

type FooterProps = React.ComponentPropsWithoutRef<typeof Menu.Label>;

const Footer = forwardRef<HTMLDivElement, Menu.LabelProps>(({ children, className, ...props }, ref) => {
  return (
    <Menu.Label
      ref={ref}
      className={cn("-mx-1 -my-1.5 mt-2 rounded-b-lg px-3.5 py-1.5 font-normal", className)}
      {...props}
    >
      {children}
    </Menu.Label>
  );
});

export { Footer };

export type { FooterProps };
