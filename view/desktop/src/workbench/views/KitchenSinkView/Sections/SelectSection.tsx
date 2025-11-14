import SelectOutlined from "@/workbench/ui/components/SelectOutlined";

import { KitchenSinkSection } from "../KitchenSinkSection";

export const SelectSection = () => {
  return (
    <KitchenSinkSection header="Select">
      <div className="flex gap-2">
        <SelectOutlined.Root>
          <SelectOutlined.Trigger placeholder="Select Outlined" />
          <SelectOutlined.Content>
            <SelectOutlined.Item value="1">Item 1</SelectOutlined.Item>
            <SelectOutlined.Item value="2">Item 2</SelectOutlined.Item>
            <SelectOutlined.Item value="3">Item 3</SelectOutlined.Item>
          </SelectOutlined.Content>
        </SelectOutlined.Root>

        <SelectOutlined.Root>
          <SelectOutlined.Trigger placeholder="Select Outlined disabled" disabled={true} />
          <SelectOutlined.Content>
            <SelectOutlined.Item value="1">Item 1</SelectOutlined.Item>
            <SelectOutlined.Item value="2">Item 2</SelectOutlined.Item>
            <SelectOutlined.Item value="3">Item 3</SelectOutlined.Item>
          </SelectOutlined.Content>
        </SelectOutlined.Root>
      </div>
    </KitchenSinkSection>
  );
};
