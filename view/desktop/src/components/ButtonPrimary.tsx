import { cva } from "class-variance-authority";

import { cn } from "@/utils";

import Button, { ButtonProps } from "../lib/ui/Button";

export interface ButtonPrimaryProps extends ButtonProps {
  size?: "md";
}

//prettier-ignore
const buttonStyles = cva(`
    background-(--moss-button-primary-solid-background)
    hover:background-(--moss-button-primary-solid-background-hover)
    active:background-(--moss-button-primary-solid-background-active)
    text-(--moss-button-primary-solid-text) 

    outline-(--moss-primary)
    
    disabled:background-(--moss-button-background-disabled)
    disabled:hover:background-(--moss-button-background-disabled-hover)
    disabled:active:background-(--moss-button-background-disabled-active)
    disabled:border-(--moss-button-border-disabled) 
    disabled:text-(--moss-button-text-disabled) 
    disabled:hover:border-(--moss-button-border-disabled-hover)
    disabled:active:border-(--moss-button-border-disabled-active)
  `,
  {
    variants:{
      size:{
        md: "h-[28px]"
      }
    }
  }
)

export const ButtonPrimary = ({ size = "md", ...props }: ButtonPrimaryProps) => {
  return <Button className={cn(buttonStyles({ size }), props.className)} {...props} />;
};

export default ButtonPrimary;
