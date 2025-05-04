import React from "react";
import type { SVGProps } from "react";
const SvgMoreVertical: React.FC<SVGProps<SVGSVGElement>> = (props) => (
  <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" fill="none" {...props}>
    <circle cx="8" cy="3" r="1" className="fill-[var(--moss-gray-6)] dark:fill-[var(--moss-gray-11)]" />
    <circle cx="8" cy="8" r="1" className="fill-[var(--moss-gray-6)] dark:fill-[var(--moss-gray-11)]" />
    <circle cx="8" cy="13" r="1" className="fill-[var(--moss-gray-6)] dark:fill-[var(--moss-gray-11)]" />
  </svg>
);
export default SvgMoreVertical;
