import { cva } from "class-variance-authority";

export const menuItemStyles = cva("flex items-center gap-1.5 rounded px-2 py-1", {
  variants: {
    disabled: {
      true: "cursor-not-allowed opacity-50",
      false: "hover:outline-hidden cursor-pointer",
    },
  },
});

export const menuContentStyles = cva("z-50 min-w-36 rounded-lg px-2 py-2 shadow-xl", {
  variants: {},
});

export const menuIconStyles = cva("size-[16px]", {
  variants: {},
});
