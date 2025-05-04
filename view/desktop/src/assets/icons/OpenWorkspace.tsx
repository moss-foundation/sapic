import React from "react";
import type { SVGProps } from "react";
const SvgOpenWorkspace: React.FC<SVGProps<SVGSVGElement>> = (props) => (
  <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" fill="none" {...props}>
    <path
      d="M2.75 2.5H6.03809C6.13578 2.5 6.23084 2.52843 6.31152 2.58105L6.38672 2.6416L8.15137 4.3584L8.29688 4.5H13C13.8284 4.5 14.5 5.17157 14.5 6V12.1338C14.4998 12.9192 13.9103 13.5 13.25 13.5H2.75C2.08968 13.5 1.50023 12.9192 1.5 12.1338V3.86621C1.50023 3.08077 2.08968 2.5 2.75 2.5Z"
      className="stroke-[var(--moss-gray-6)] dark:stroke-[var(--moss-gray-11)]"
    />
  </svg>
);
export default SvgOpenWorkspace;
