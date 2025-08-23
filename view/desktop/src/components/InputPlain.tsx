import { cva } from "class-variance-authority";

import Input, { InputProps } from "@/lib/ui/Input";
import { cn } from "@/utils";

//prettier-ignore
const inputStyles = cva(`
    placeholder-(--moss-controls-placeholder)
    background-(--moss-input-bg-plain)
    text-(--moss-controls-plain-text)
    has-data-invalid:text-(--moss-error)
    py-[5px] px-2
  `, 
);

const iconsStyles = cva("size-4");

export const InputPlain = ({ className, iconClassName, ...props }: InputProps) => {
  return <Input className={cn(inputStyles(), className)} iconClassName={cn(iconsStyles(), iconClassName)} {...props} />;
};

export default InputPlain;
