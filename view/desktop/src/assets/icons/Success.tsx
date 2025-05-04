import React from "react";
import type { SVGProps } from "react";
const SvgSuccess: React.FC<SVGProps<SVGSVGElement>> = (props) => (
  <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" fill="none" {...props}>
    <circle cx="8" cy="8" r="7" className="fill-[var(--moss-green-6)] dark:fill-[var(--moss-green-5)]" />
    <path
      d="M4.5 8L7 10.5L11.5 6"
      strokeWidth="2"
      strokeLinecap="round"
      strokeLinejoin="round"
      className="stroke-[var(--moss-gray-14)] dark:stroke-[var(--moss-input-bg-outlined)]"
    />
  </svg>
);
export default SvgSuccess;
