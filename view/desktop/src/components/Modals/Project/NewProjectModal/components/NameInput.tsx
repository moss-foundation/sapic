import { forwardRef } from "react";

import { VALID_NAME_PATTERN } from "@/constants/validation";
import Input, { InputProps } from "@/lib/ui/Input";

interface NameInputProps extends InputProps {
  name: string;
  setName: (name: string) => void;
}

export const NameInput = forwardRef<HTMLInputElement, NameInputProps>(({ name, setName, ...props }, ref) => {
  return (
    <div className="col-span-2 grid grid-cols-subgrid items-center gap-y-1.5">
      <div>Name:</div>
      <Input
        ref={ref}
        value={name}
        className="max-w-72"
        onChange={(e) => setName(e.target.value)}
        pattern={VALID_NAME_PATTERN}
        placeholder="New Project"
        required
        intent="outlined"
        {...props}
      />
      <p className="text-(--moss-secondary-foreground) col-start-2 max-w-72 text-sm">{`Invalid filename characters (e.g. / \ : * ? " < > |) will be escaped`}</p>
    </div>
  );
});
