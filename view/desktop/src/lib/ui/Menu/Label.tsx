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
    className={cn("text-(--moss-primary-foreground) px-3 py-2 text-center", className)}
    {...props}
  />
));

export { Label };

export type { LabelProps };
