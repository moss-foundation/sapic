import SelectOutlined from "@/workbench/ui/components/SelectOutlined";
import { AccountInfo } from "@repo/base";

interface AccountSelectProps {
  accounts: AccountInfo[];
  onValueChange: (value: string) => void;
}

export const AccountSelect = ({ accounts, onValueChange }: AccountSelectProps) => {
  const isDisabled = accounts.length === 0;

  return (
    <div className="col-span-2 grid grid-cols-subgrid items-center">
      <div>Account:</div>

      <SelectOutlined.Root onValueChange={onValueChange}>
        <SelectOutlined.Trigger disabled={isDisabled} />
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
