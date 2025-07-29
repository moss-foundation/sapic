import { cva } from "class-variance-authority";
import { forwardRef } from "react";

import Input, { InputProps } from "@/lib/ui/Input";
import { cn } from "@/utils";

interface InputPlainProps extends InputProps {
  size?: "sm" | "md";
}

//prettier-ignore
const inputStyles = cva(`
    placeholder-(--moss-controls-placeholder)
    background-(--moss-controls-outlined-bg) 
    border border-(--moss-controls-outlined-border)
    text-(--moss-controls-outlined-text)
    has-data-invalid:border-(--moss-error)
    has-[input:focus-within]:outline-(--moss-primary)
    has-[input:focus-within]:has-data-invalid:outline-(--moss-error)
    has-[input:focus-within]:outline-2
    font-normal
  `, 
{
  variants: {
    size: {
        xs: "h-6 px-1.5",
        sm: "h-7 px-2",
        md: "h-9 px-2",
    },
  },
});

const iconsStyles = cva("", {
  variants: {
    size: {
      sm: "size-4",
      md: "size-4",
    },
  },
});

export const InputOutlined = forwardRef<HTMLInputElement, InputPlainProps>(
  ({ size = "sm", className, iconClassName, ...props }, ref) => {
    return (
      <Input
        ref={ref}
        className={cn(inputStyles({ size }), className)}
        iconClassName={cn(iconsStyles({ size }), iconClassName)}
        {...props}
      />
    );
  }
);

export default InputOutlined;
