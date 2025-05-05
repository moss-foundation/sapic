import { cn } from "@/utils";

import Button, { ButtonProps } from "../lib/ui/Button";

export const ButtonPrimary = ({ ...props }: ButtonProps) => {
  return (
    <Button
      className={cn(
        "background-(--moss-button-primary-solid-background) hover:background-(--moss-button-primary-solid-background-hover) active:background-(--moss-button-primary-solid-background-active) text-(--moss-button-primary-solid-text)"
      )}
      {...props}
    />
  );
};

export default ButtonPrimary;
