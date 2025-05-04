import React from "react";
import type { SVGProps } from "react";
const SvgKey: React.FC<SVGProps<SVGSVGElement>> = (props) => (
  <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" fill="none" {...props}>
    <path
      d="M8.96905 7.5C8.723 5.52684 7.03981 4 5 4C2.79086 4 1 5.79086 1 8C1 10.2091 2.79086 12 5 12C7.03981 12 8.723 10.4732 8.96905 8.5H14.5C14.7761 8.5 15 8.27614 15 8C15 7.72386 14.7761 7.5 14.5 7.5H14V6C14 5.72386 13.7761 5.5 13.5 5.5C13.2239 5.5 13 5.72386 13 6V7.5H12V6C12 5.72386 11.7761 5.5 11.5 5.5C11.2239 5.5 11 5.72386 11 6V7.5H8.96905ZM5 5C6.65685 5 8 6.34315 8 8C8 9.65685 6.65685 11 5 11C3.34315 11 2 9.65685 2 8C2 6.34315 3.34315 5 5 5Z"
      fillRule="evenodd"
      clipRule="evenodd"
      className="fill-[var(--moss-gray-4)] dark:fill-[var(--moss-gray-11)]"
    />
  </svg>
);
export default SvgKey;
