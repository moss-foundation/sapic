import { cva } from "class-variance-authority";

import { cn } from "@/utils";

import Button, { ButtonProps } from "../lib/ui/Button";

export interface ButtonNeutralOutlinedProps extends ButtonProps {
  size?: "md";
}

//prettier-ignore
const buttonStyles = cva(`
    background-(--moss-button-neutral-outlined-background)
    hover:background-(--moss-button-neutral-outlined-background-hover)
    active:background-(--moss-button-neutral-outlined-background-active)
    border border-(--moss-button-neutral-outlined-border)
    hover:border-(--moss-button-neutral-outlined-border-hover)
    active:border-(--moss-button-neutral-outlined-border-active)
    text-(--moss-button-neutral-outlined-text)

    outline-(--moss-primary)

    disabled:background-(--moss-button-background-disabled)
    disabled:hover:background-(--moss-button-background-disabled-hover)
    disabled:active:background-(--moss-button-background-disabled-active)
    disabled:border-(--moss-button-border-disabled) 
    disabled:hover:border-(--moss-button-border-disabled-hover)
    disabled:active:border-(--moss-button-border-disabled-active)
    disabled:text-(--moss-button-text-disabled) 
  `,
  {
    variants:{
      size:{
        md: "h-[28px]"
      }
    }
  }
)

export const ButtonNeutralOutlined = ({ size = "md", className, ...props }: ButtonNeutralOutlinedProps) => {
  return <Button className={cn(buttonStyles({ size }), className)} {...props} />;
};

export default ButtonNeutralOutlined;
