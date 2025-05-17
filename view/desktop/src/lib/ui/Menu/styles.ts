import { cva } from "class-variance-authority";

export const menuItemStyles = cva("flex items-center gap-2.5 rounded py-0.5 pr-3 pl-4", {
  variants: {
    disabled: {
      true: "cursor-not-allowed opacity-50",
      false: "cursor-pointer hover:outline-hidden",
    },
  },
});

export const menuContentStyles = cva("z-50 min-w-36 rounded-lg px-1 py-1.5 shadow-lg", {
  variants: {},
});
