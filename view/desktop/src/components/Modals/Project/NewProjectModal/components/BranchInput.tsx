import { VALID_NAME_PATTERN } from "@/constants/validation";
import Input, { InputProps } from "@/lib/ui/Input";

interface BranchInputProps extends InputProps {
  branch: string;
  setBranch: (branch: string) => void;
}

export const BranchInput = ({ branch, setBranch, ...props }: BranchInputProps) => {
  return (
    <div className="col-span-2 grid grid-cols-subgrid items-center">
      <div>Branch:</div>
      <Input
        value={branch}
        className="max-w-72"
        onChange={(e) => setBranch(e.target.value)}
        pattern={VALID_NAME_PATTERN}
        required
        intent="outlined"
        {...props}
      />
    </div>
  );
};
