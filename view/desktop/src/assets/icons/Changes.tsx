import React from "react";
import type { SVGProps } from "react";
const SvgChanges: React.FC<SVGProps<SVGSVGElement>> = (props) => (
  <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" fill="none" {...props}>
    <rect
      x="2"
      y="5"
      width="7"
      height="1"
      rx="0.5"
      className="fill-[var(--moss-gray-6)] dark:fill-[var(--moss-gray-11)]"
    />
    <rect
      x="2"
      y="8"
      width="5"
      height="1"
      rx="0.5"
      className="fill-[var(--moss-gray-6)] dark:fill-[var(--moss-gray-11)]"
    />
    <rect
      x="2"
      y="2"
      width="12"
      height="1"
      rx="0.5"
      className="fill-[var(--moss-gray-6)] dark:fill-[var(--moss-gray-11)]"
    />
    <path
      d="M8.5 14.5L10.5 12.5L8.5 10.5M5.5 12.5H10M12.5 10.5L10.5 8.5L12.5 6.5M15.5 8.5H11"
      strokeLinecap="round"
      strokeLinejoin="round"
      className="stroke-[var(--moss-blue-4)] dark:stroke-[var(--moss-blue-8)]"
    />
  </svg>
);
export default SvgChanges;
