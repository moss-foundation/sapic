import React from "react";
import type { SVGProps } from "react";
const SvgCollapseAll: React.FC<SVGProps<SVGSVGElement>> = (props) => (
  <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" fill="none" {...props}>
    <path
      d="M4.5 2.5L8 6L11.5 2.5"
      strokeLinecap="round"
      strokeLinejoin="round"
      className="stroke-[var(--moss-gray-6)] dark:stroke-[var(--moss-gray-11)]"
    />
    <path
      d="M4.5 13.5L8 10L11.5 13.5"
      strokeLinecap="round"
      strokeLinejoin="round"
      className="stroke-[var(--moss-gray-6)] dark:stroke-[var(--moss-gray-11)]"
    />
  </svg>
);
export default SvgCollapseAll;
