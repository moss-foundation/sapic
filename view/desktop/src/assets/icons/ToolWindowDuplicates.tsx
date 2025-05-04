import React from "react";
import type { SVGProps } from "react";
const SvgToolWindowDuplicates: React.FC<SVGProps<SVGSVGElement>> = (props) => (
  <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" fill="none" {...props}>
    <rect
      x="1.5"
      y="5.5"
      width="9"
      height="9"
      rx="1.5"
      className="stroke-[var(--moss-gray-6)] dark:stroke-[var(--moss-gray-11)]"
    />
    <path
      d="M4.5 3.5H11C11.8284 3.5 12.5 4.17157 12.5 5V11.5"
      strokeLinecap="round"
      className="stroke-[var(--moss-gray-6)] dark:stroke-[var(--moss-gray-11)]"
    />
    <path
      d="M6.5 1.5H13C13.8284 1.5 14.5 2.17157 14.5 3V9.5"
      strokeLinecap="round"
      className="stroke-[var(--moss-gray-6)] dark:stroke-[var(--moss-gray-11)]"
    />
  </svg>
);
export default SvgToolWindowDuplicates;
