//TODO this whole file should be removed, because this icons are just stand in for the real ones
import { SVGProps } from "react";

export const FolderIcon = ({ ...props }: SVGProps<SVGSVGElement>) => (
  <svg width="16" height="16" viewBox="0 0 16 16" fill="none" xmlns="http://www.w3.org/2000/svg" {...props}>
    <path
      d="M8.10584 4.34613L8.25344 4.5H8.46667H13C13.8284 4.5 14.5 5.17157 14.5 6V12.1333C14.5 12.9529 13.932 13.5 13.3667 13.5H2.63333C2.06804 13.5 1.5 12.9529 1.5 12.1333V3.86667C1.5 3.04707 2.06804 2.5 2.63333 2.5H6.1217C6.25792 2.5 6.38824 2.55557 6.48253 2.65387L8.10584 4.34613Z"
      fill="#EBECF0"
      stroke="#6C707E"
    />
  </svg>
);

export const FileIcon = ({ ...props }: SVGProps<SVGSVGElement>) => (
  <svg
    xmlns="http://www.w3.org/2000/svg"
    width="16"
    height="16"
    viewBox="0 0 24 24"
    fill="none"
    stroke="currentColor"
    strokeWidth="2"
    strokeLinecap="round"
    strokeLinejoin="round"
    {...props}
  >
    <path d="M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7Z" />
    <path d="M14 2v4a2 2 0 0 0 2 2h4" />
  </svg>
);

export const ChevronRightIcon = ({ ...props }: SVGProps<SVGSVGElement>) => (
  <svg width="16" height="16" viewBox="0 0 16 16" fill="none" xmlns="http://www.w3.org/2000/svg" {...props}>
    <path d="M6 11.5L9.5 8L6 4.5" stroke="#818594" strokeLinecap="round" />
  </svg>
);

export const ExpandAllIcon = ({ ...props }: SVGProps<SVGSVGElement>) => (
  <svg width="16" height="16" viewBox="0 0 16 16" fill="none" xmlns="http://www.w3.org/2000/svg" {...props}>
    <path d="M4.5 6L8 2.5L11.5 6" stroke="#717171" strokeLinecap="round" strokeLinejoin="round" />
    <path d="M4.5 10L8 13.5L11.5 10" stroke="#717171" strokeLinecap="round" strokeLinejoin="round" />
  </svg>
);

export const CollapseAllIcon = ({ ...props }: SVGProps<SVGSVGElement>) => (
  <svg width="16" height="16" viewBox="0 0 16 16" fill="none" xmlns="http://www.w3.org/2000/svg" {...props}>
    <path d="M4.5 2.5L8 6L11.5 2.5" stroke="#6C707E" strokeLinecap="round" strokeLinejoin="round" />
    <path d="M4.5 13.5L8 10L11.5 13.5" stroke="#6C707E" strokeLinecap="round" strokeLinejoin="round" />
  </svg>
);

export const TreeRootDetailIcon = ({ ...props }: SVGProps<SVGSVGElement>) => (
  <svg width="16" height="16" viewBox="0 0 16 16" fill="none" xmlns="http://www.w3.org/2000/svg" {...props}>
    <circle cx="8" cy="3" r="1" fill="#717171" />
    <circle cx="8" cy="8" r="1" fill="#717171" />
    <circle cx="8" cy="13" r="1" fill="#717171" />
  </svg>
);
