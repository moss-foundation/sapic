import { cn } from "@/utils";

import Button, { ButtonProps } from "../lib/ui/Button";

export const ButtonNeutral = ({ ...props }: ButtonProps) => {
  return (
    <Button
      className={cn(
        `background-(--moss-button-neutral-outlined-background) hover:background-(--moss-button-neutral-outlined-background-hover) active:background-(--moss-button-neutral-outlined-background-active) border border-(--moss-button-neutral-outlined-border) text-(--moss-button-neutral-outlined-text) hover:border-(--moss-button-neutral-outlined-border-hover) active:border-(--moss-button-neutral-outlined-border-active)`
      )}
      {...props}
    />
  );
};

export default ButtonNeutral;
