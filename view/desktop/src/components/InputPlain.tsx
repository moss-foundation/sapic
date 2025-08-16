import { cva } from "class-variance-authority";

import Input, { InputProps } from "@/lib/ui/Input";
import { cn } from "@/utils";

interface InputPlainProps extends InputProps {
  size?: "sm" | "md";
}

//prettier-ignore
const inputStyles = cva(`
    placeholder-(--moss-controls-placeholder)
    background-(--moss-input-bg-plain)
    text-(--moss-controls-plain-text)
    has-data-invalid:text-(--moss-error)
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

export const InputPlain = ({ size = "md", ...props }: InputPlainProps) => {
  return (
    <Input
      className={cn(inputStyles({ size }), props.className)}
      iconClassName={cn(iconsStyles({ size }), props.iconClassName)}
      {...props}
    />
  );
};

export default InputPlain;
