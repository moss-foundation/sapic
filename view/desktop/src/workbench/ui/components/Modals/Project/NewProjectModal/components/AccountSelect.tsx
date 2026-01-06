import { useEffect, useState } from "react";

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

  useEffect(() => {
    if (accounts.length === 1 && selectedValue !== accounts[0].id) {
      const accountId = accounts[0].id;
      setSelectedValue(accountId);
      onValueChange(accountId);
    }
  }, [accounts, selectedValue, onValueChange]);

  const handleValueChange = (value: string) => {
    setSelectedValue(value);
    onValueChange(value);
  };

  return (
    <div className="col-span-2 grid grid-cols-subgrid items-center">
      <div>Account:</div>

      <SelectOutlined.Root value={selectedValue} onValueChange={handleValueChange} disabled={isDisabled}>
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
