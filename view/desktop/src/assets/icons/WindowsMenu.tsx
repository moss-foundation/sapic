import React from "react";
import type { SVGProps } from "react";
const SvgWindowsMenu: React.FC<SVGProps<SVGSVGElement>> = (props) => (
  <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" fill="none" {...props}>
    <rect
      y="3"
      width="16"
      height="1.5"
      rx="0.75"
      className="fill-[var(--moss-gray-6)] dark:fill-[var(--moss-gray-11)]"
    />
    <rect
      y="7.5"
      width="16"
      height="1.5"
      rx="0.75"
      className="fill-[var(--moss-gray-6)] dark:fill-[var(--moss-gray-11)]"
    />
    <rect
      y="12"
      width="16"
      height="1.5"
      rx="0.75"
      className="fill-[var(--moss-gray-6)] dark:fill-[var(--moss-gray-11)]"
    />
  </svg>
);
export default SvgWindowsMenu;
