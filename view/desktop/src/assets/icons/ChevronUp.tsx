import React from "react";
import type { SVGProps } from "react";
const SvgChevronUp: React.FC<SVGProps<SVGSVGElement>> = (props) => (
  <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" fill="none" {...props}>
    <path
      d="M12.5 10.25L8 5.75L3.5 10.25"
      strokeLinecap="round"
      className="stroke-[var(--moss-gray-6)] dark:stroke-[var(--moss-gray-11)]"
    />
  </svg>
);
export default SvgChevronUp;
