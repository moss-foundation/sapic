import { cva } from "class-variance-authority";

export const actionMenuStyles = cva("hover:background-(--moss-secondary-background-hover) cursor-pointer", {
  variants: {},
});

export const actionMenuContentStyles = cva(
  "background-(--moss-primary-background) border-(--moss-border) w-60 border",
  {
    variants: {},
  }
);
