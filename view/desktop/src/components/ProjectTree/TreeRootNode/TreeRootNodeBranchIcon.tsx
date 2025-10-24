type BranchIcon = "up" | "down";

interface BranchIconProps extends React.SVGProps<SVGSVGElement> {
  icon: BranchIcon;
}

export const TreeRootNodeBranchIcon = ({ icon, ...props }: BranchIconProps) => {
  switch (icon) {
    case "down":
      return (
        <svg width="12" height="12" viewBox="0 0 12 12" fill="none" xmlns="http://www.w3.org/2000/svg" {...props}>
          <path
            d="M6.00227 2.48602L5.96359 8.84986M8.80485 6.74576L5.95929 9.55695L3.1481 6.71139"
            stroke="#3574F0"
            strokeLinecap="round"
            strokeLinejoin="round"
          />
        </svg>
      );

    case "up":
      return (
        <svg width="12" height="12" viewBox="0 0 12 12" fill="none" xmlns="http://www.w3.org/2000/svg" {...props}>
          <path
            d="M5.97901 9.54521L5.99638 3.18125M3.16215 5.29484L5.99831 2.47416L8.81899 5.31029"
            stroke="#369650"
            strokeLinecap="round"
            strokeLinejoin="round"
          />
        </svg>
      );
  }
};
