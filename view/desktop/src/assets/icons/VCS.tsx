import React from "react";
import type { SVGProps } from "react";
const SvgVCS: React.FC<SVGProps<SVGSVGElement>> = (props) => (
  <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" fill="none" {...props}>
    <path
      d="M4.5 11.5H8.5C9.60457 11.5 10.5 10.6046 10.5 9.5V9.5V8"
      className="stroke-[var(--moss-gray-6)] dark:stroke-[var(--moss-gray-11)]"
    />
    <path
      d="M4.5 6.5L4.5 14.5"
      strokeLinecap="round"
      strokeLinejoin="round"
      className="stroke-[var(--moss-gray-6)] dark:stroke-[var(--moss-gray-11)]"
    />
    <circle cx="10.5" cy="6" r="2" className="stroke-[var(--moss-gray-6)] dark:stroke-[var(--moss-gray-11)]" />
    <circle cx="4.5" cy="4" r="2" className="stroke-[var(--moss-gray-6)] dark:stroke-[var(--moss-gray-11)]" />
  </svg>
);
export default SvgVCS;
