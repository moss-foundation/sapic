import React from "react";

import { cn } from "@/utils";
import * as MenuPrimitive from "@radix-ui/react-menu";

type LabelProps = React.ComponentPropsWithoutRef<typeof MenuPrimitive.Label>;

const Label = React.forwardRef<
  React.ElementRef<typeof MenuPrimitive.Label>,
  React.ComponentPropsWithoutRef<typeof MenuPrimitive.Label>
>(({ className, ...props }, ref) => (
  <MenuPrimitive.Label
    ref={ref}
    className={cn("px-3 py-2 text-center font-medium text-(--moss-primary-text)", className)}
    {...props}
  />
));

export { Label };

export type { LabelProps };
