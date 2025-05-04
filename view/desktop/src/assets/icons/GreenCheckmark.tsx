import React from "react";
import type { SVGProps } from "react";
const SvgGreenCheckmark: React.FC<SVGProps<SVGSVGElement>> = (props) => (
  <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" fill="none" {...props}>
    <path
      d="M2.5 8.25L6 11.75L13.5 4.25"
      strokeWidth="1.5"
      strokeLinecap="round"
      strokeLinejoin="round"
      className="stroke-[var(--moss-green-5)] dark:stroke-[var(--moss-green-5)]"
    />
  </svg>
);
export default SvgGreenCheckmark;
