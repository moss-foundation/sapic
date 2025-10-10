import { cva } from "class-variance-authority";
import { forwardRef } from "react";

import Input, { InputProps } from "@/lib/ui/Input";
import { cn } from "@/utils";

//prettier-ignore
const inputStyles = cva(`
    placeholder-(--moss-controls-placeholder)
    border border-(--moss-controls-outlined-border)
    text-(--moss-controls-outlined-text)
    has-data-invalid:border-(--moss-error)
    font-normal
    py-[5px] px-2
  `,
  {
  variants: {
    contrast: {
      true: "background-(--moss-controls-outlined-bg-contrast)",
      false: "background-(--moss-controls-outlined-bg)",
    },
  },
});

const iconsStyles = cva("size-4");

interface InputOutlinedProps extends InputProps {
  contrast?: boolean;
}

export const InputOutlined = forwardRef<HTMLInputElement, InputOutlinedProps>(
  ({ className, contrast = false, iconClassName, ...props }, ref) => {
    return (
      <Input
        ref={ref}
        className={cn(inputStyles({ contrast }), className)}
        iconClassName={cn(iconsStyles(), iconClassName)}
        {...props}
      />
    );
  }
);

export default InputOutlined;
