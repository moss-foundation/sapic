import { cva } from "class-variance-authority";

import { cn } from "@/utils";

import Button, { ButtonProps } from "../lib/ui/Button";

export interface ButtonDangerProps extends ButtonProps {
  size?: "md";
}

//prettier-ignore
const buttonStyles = cva(`
    background-(--moss-button-background-danger)
    hover:background-(--moss-button-background-danger-hover)
    text-(--moss-button-text-danger) 

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

export const ButtonDanger = ({ size = "md", ...props }: ButtonDangerProps) => {
  return <Button className={cn(buttonStyles({ size }), props.className)} {...props} />;
};

export default ButtonDanger;
