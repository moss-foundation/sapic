import { useState } from "react";

import SelectOutlined from "@/workbench/ui/components/SelectOutlined";
import { AccountInfo } from "@repo/base";

interface AccountSelectProps {
  accounts: AccountInfo[];
  onValueChange: (value: string) => void;
  disabled?: boolean;
}

export const AccountSelect = ({ accounts, onValueChange, disabled }: AccountSelectProps) => {
  const [selectedValue, setSelectedValue] = useState<string>("");

  const isDisabled = disabled || accounts.length === 0;

  const effectiveValue = accounts.length === 1 ? accounts[0].id : selectedValue;

  const handleValueChange = (value: string) => {
    setSelectedValue(value);
    onValueChange(value);
  };

  return (
    <div className="col-span-2 grid grid-cols-subgrid items-center">
      <div>Account:</div>

      <SelectOutlined.Root value={effectiveValue} onValueChange={handleValueChange} disabled={isDisabled}>
        <SelectOutlined.Trigger disabled={isDisabled} className="w-full max-w-72" />
        <SelectOutlined.Content>
          {accounts.map((account) => (
            <SelectOutlined.Item key={account.id} value={account.id}>
              {account.username}
            </SelectOutlined.Item>
          ))}
        </SelectOutlined.Content>
      </SelectOutlined.Root>
    </div>
  );
};
