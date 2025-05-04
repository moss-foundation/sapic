import React from "react";
import type { SVGProps } from "react";
const SvgClose: React.FC<SVGProps<SVGSVGElement>> = (props) => (
  <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" fill="none" {...props}>
    <path
      d="M2.5 13.5L13.5 2.5M13.5 13.5L2.5 2.5"
      strokeLinecap="round"
      className="stroke-[var(--moss-gray-6)] dark:stroke-[var(--moss-gray-11)]"
    />
  </svg>
);
export default SvgClose;
