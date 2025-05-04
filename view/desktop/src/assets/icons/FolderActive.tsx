import React from "react";
import type { SVGProps } from "react";
const SvgFolderActive: React.FC<SVGProps<SVGSVGElement>> = (props) => (
  <div {...props}>
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" fill="none" className="block dark:hidden">
      <path
        d="M2.63379 2.5H6.12207C6.22394 2.50007 6.32263 2.53103 6.40527 2.58789L6.48242 2.6543L8.10547 4.3457L8.25391 4.5H13C13.8284 4.5 14.5 5.17157 14.5 6V12.1338C14.4998 12.9531 13.9314 13.5 13.3662 13.5H2.63379C2.0686 13.5 1.50021 12.9531 1.5 12.1338V3.86621C1.5002 3.09811 1.99927 2.56898 2.52734 2.50586L2.63379 2.5Z"
        className="fill-[var(--moss-blue-4)] stroke-[var(--moss-blue-4)]"
      />
      <path
        d="M2.63379 2.5H6.12207C6.22394 2.50007 6.32263 2.53103 6.40527 2.58789L6.48242 2.6543L8.10547 4.3457L8.25391 4.5H13C13.8284 4.5 14.5 5.17157 14.5 6V12.1338C14.4998 12.9531 13.9314 13.5 13.3662 13.5H2.63379C2.0686 13.5 1.50021 12.9531 1.5 12.1338V3.86621C1.5002 3.09811 1.99927 2.56898 2.52734 2.50586L2.63379 2.5Z"
        fill="#3574F0"
        stroke="#3574F0"
      />
    </svg>
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" fill="none" className="hidden dark:block">
      <path
        d="M2.63379 2.5H6.12207C6.22394 2.50007 6.32263 2.53103 6.40527 2.58789L6.48242 2.6543L8.10547 4.3457L8.25391 4.5H13C13.8284 4.5 14.5 5.17157 14.5 6V12.1338C14.4998 12.9531 13.9314 13.5 13.3662 13.5H2.63379C2.0686 13.5 1.50021 12.9531 1.5 12.1338V3.86621C1.5002 3.09811 1.99927 2.56898 2.52734 2.50586L2.63379 2.5Z"
        className="fill-[var(--moss-icon-primary-background-active)] stroke-[var(--moss-icon-primary-background-active)]"
      />
    </svg>
  </div>
);
export default SvgFolderActive;
