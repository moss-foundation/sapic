import React from "react";
import type { SVGProps } from "react";
const SvgCheckboxIndicator: React.FC<SVGProps<SVGSVGElement>> = (props) => (
  <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 16 16" {...props}>
    <rect
      width="16"
      height="16"
      rx="3"
      className="fill-[var(--moss-blue-4)] dark:fill-[var(--moss-icon-primary-background-active)]"
    />
    <path
      d="m4 8.5 3 3L12.5 5"
      strokeLinecap="round"
      strokeLinejoin="round"
      strokeWidth="2"
      className="stroke-[var(--moss-gray-14)] dark:stroke-[var(--moss-input-bg-outlined)]"
    />
  </svg>
);
export default SvgCheckboxIndicator;
