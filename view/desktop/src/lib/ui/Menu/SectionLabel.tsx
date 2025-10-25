import { forwardRef } from "react";

import { Menu } from "@/lib/ui";
import { cn } from "@/utils";

type SectionLabelProps = React.ComponentPropsWithoutRef<typeof Menu.Label>;

const SectionLabel = forwardRef<HTMLDivElement, Menu.LabelProps>(({ children, className, ...props }, ref) => {
  return (
    <Menu.Label ref={ref} className={cn("-ml-1 px-3 py-1 text-left", className)} {...props}>
      {children}
    </Menu.Label>
  );
});

export { SectionLabel };

export type { SectionLabelProps };
