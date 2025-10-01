import { cva } from "class-variance-authority";
import { forwardRef } from "react";

import Input, { InputProps } from "@/lib/ui/Input";
import { cn } from "@/utils";

//prettier-ignore
const inputStyles = cva(`
    placeholder-(--moss-controls-placeholder)
    background-(--moss-controls-outlined-bg) 
    border border-(--moss-controls-outlined-border)
    text-(--moss-controls-outlined-text)
    has-data-invalid:border-(--moss-error)
    font-normal
    py-[5px] px-2
  `, 
);

const iconsStyles = cva("size-4");

export const InputOutlined = forwardRef<HTMLInputElement, InputProps>(({ className, iconClassName, ...props }, ref) => {
  return (
    <Input
      ref={ref}
      className={cn(inputStyles(), className)}
      iconClassName={cn(iconsStyles(), iconClassName)}
      {...props}
    />
  );
});

export default InputOutlined;
