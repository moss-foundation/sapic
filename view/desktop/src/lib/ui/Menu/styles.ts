import { cva } from "class-variance-authority";

export const menuItemStyles = cva("flex items-center gap-1.5 rounded py-[3px] pl-4 pr-3", {
  variants: {
    disabled: {
      true: "cursor-not-allowed opacity-50",
      false: "hover:outline-hidden cursor-pointer",
    },
  },
});

export const menuContentStyles = cva("z-50 min-w-36 rounded-lg px-1 py-1.5 shadow-lg", {
  variants: {},
});

export const menuIconStyles = cva("size-[16px]", {
  variants: {},
});
