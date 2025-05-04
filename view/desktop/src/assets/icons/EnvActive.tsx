import React from "react";
import type { SVGProps } from "react";
const SvgEnvActive: React.FC<SVGProps<SVGSVGElement>> = (props) => (
  <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" fill="none" {...props}>
    <path
      d="M3.5 13.5H2.5C1.94772 13.5 1.5 13.0523 1.5 12.5V8V3.5C1.5 2.94772 1.94772 2.5 2.5 2.5H3.5"
      strokeLinecap="round"
      className="stroke-[var(--moss-blue-4)] dark:stroke-[var(--moss-blue-8)]"
    />
    <path
      d="M12.5 13.5H13.5C14.0523 13.5 14.5 13.0523 14.5 12.5V3.5C14.5 2.94771 14.0523 2.5 13.5 2.5H12.5"
      strokeLinecap="round"
      className="stroke-[var(--moss-blue-4)] dark:stroke-[var(--moss-blue-8)]"
    />
    <path
      d="M7.03516 4.39941C7.53458 3.89999 8.3256 3.86857 8.86133 4.30566L8.96484 4.39941L11.6006 7.03516C12.1 7.53458 12.1314 8.3256 11.6943 8.86133L11.6006 8.96484L8.96484 11.6006C8.46542 12.1 7.6744 12.1314 7.13867 11.6943L7.03516 11.6006L4.39941 8.96484C3.89999 8.46542 3.86857 7.6744 4.30566 7.13867L4.39941 7.03516L7.03516 4.39941Z"
      className="fill-[var(--moss-blue-12)] stroke-[var(--moss-blue-4)] dark:fill-[var(--moss-blue-1)] dark:stroke-[var(--moss-blue-8)]"
    />
  </svg>
);
export default SvgEnvActive;
