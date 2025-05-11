import { useState } from "react";

import { ActionButton, ActionMenuRadix } from "@/components";
import { useCollectionsStore } from "@/store/collections";

export const SidebarHeader = ({ title }: { title: string }) => {
  const { collapseAll } = useCollectionsStore();

  return (
    <div className="background-(--moss-secondary-background) relative flex items-center justify-between px-2 py-[5px] text-(--moss-primary-text) uppercase">
      <div className="w-max items-center overflow-hidden text-xs text-ellipsis whitespace-nowrap text-(--moss-secondary-text)">
        {title}
      </div>

      <div className="flex grow justify-end">
        <ActionButton icon="Add" />
        <ActionButton icon="CollapseAll" onClick={collapseAll} />
        {/* <ActionButton icon="Import" /> */}
        {/* <ActionButton icon="MoreHorizontal" /> */}
        <ExampleContextMenuRadix />
        <ExampleDropdownMenu />
      </div>
    </div>
  );
};

export default SidebarHeader;

const ExampleDropdownMenu = () => {
  const [isChecked, setIsChecked] = useState(false);
  const [radioValue, setRadioValue] = useState("option1");

  return (
    <ActionMenuRadix.Root>
      <ActionMenuRadix.Trigger asChild>
        <ActionButton icon="MoreHorizontal" />
      </ActionMenuRadix.Trigger>

      <ActionMenuRadix.Portal>
        <ActionMenuRadix.Content>
          <ActionMenuRadix.Item onSelect={() => console.log("Item 1 selected")}>Item 1</ActionMenuRadix.Item>
          <ActionMenuRadix.Item onSelect={() => console.log("Item 2 selected")}>Item 2</ActionMenuRadix.Item>

          <ActionMenuRadix.Separator />

          <ActionMenuRadix.CheckboxItem checked={isChecked} onCheckedChange={setIsChecked}>
            Check me
          </ActionMenuRadix.CheckboxItem>

          <ActionMenuRadix.Separator />

          <ActionMenuRadix.RadioGroup value={radioValue} onValueChange={setRadioValue}>
            <ActionMenuRadix.RadioItem checked={radioValue === "option1"} value="option1">
              Option 1
            </ActionMenuRadix.RadioItem>
            <ActionMenuRadix.RadioItem checked={radioValue === "option2"} value="option2">
              Option 2
            </ActionMenuRadix.RadioItem>
          </ActionMenuRadix.RadioGroup>

          <ActionMenuRadix.Separator />

          <ActionMenuRadix.Sub>
            <ActionMenuRadix.SubTrigger>Submenu</ActionMenuRadix.SubTrigger>
            <ActionMenuRadix.SubContent>
              <ActionMenuRadix.Item hideIcon onSelect={() => console.log("Sub Item 1 selected")}>
                Sub Item 1
              </ActionMenuRadix.Item>
              <ActionMenuRadix.Item hideIcon onSelect={() => console.log("Sub Item 2 selected")}>
                Sub Item 2
              </ActionMenuRadix.Item>
            </ActionMenuRadix.SubContent>
          </ActionMenuRadix.Sub>
        </ActionMenuRadix.Content>
      </ActionMenuRadix.Portal>
    </ActionMenuRadix.Root>
  );
};

const ExampleContextMenuRadix = () => {
  const [isChecked, setIsChecked] = useState(false);
  const [radioValue, setRadioValue] = useState("option1");

  return (
    <ActionMenuRadix.Root>
      <ActionMenuRadix.Trigger asChild openOnRightClick>
        <ActionButton icon="Import" />
      </ActionMenuRadix.Trigger>

      <ActionMenuRadix.Portal>
        <ActionMenuRadix.Content>
          <ActionMenuRadix.Item onSelect={() => console.log("Item 1 selected")}>Item 1</ActionMenuRadix.Item>
          <ActionMenuRadix.Item onSelect={() => console.log("Item 2 selected")}>Item 2</ActionMenuRadix.Item>

          <ActionMenuRadix.Separator />

          <ActionMenuRadix.CheckboxItem checked={isChecked} onCheckedChange={setIsChecked}>
            Check me
          </ActionMenuRadix.CheckboxItem>

          <ActionMenuRadix.Separator />

          <ActionMenuRadix.RadioGroup value={radioValue} onValueChange={setRadioValue}>
            <ActionMenuRadix.RadioItem checked={radioValue === "option1"} value="option1">
              Option 1
            </ActionMenuRadix.RadioItem>
            <ActionMenuRadix.RadioItem checked={radioValue === "option2"} value="option2">
              Option 2
            </ActionMenuRadix.RadioItem>
          </ActionMenuRadix.RadioGroup>

          <ActionMenuRadix.Separator />

          <ActionMenuRadix.Sub>
            <ActionMenuRadix.SubTrigger>Submenu</ActionMenuRadix.SubTrigger>
            <ActionMenuRadix.SubContent>
              <ActionMenuRadix.Item hideIcon onSelect={() => console.log("Sub Item 1 selected")}>
                Sub Item 1
              </ActionMenuRadix.Item>
              <ActionMenuRadix.Item hideIcon onSelect={() => console.log("Sub Item 2 selected")}>
                Sub Item 2
              </ActionMenuRadix.Item>
            </ActionMenuRadix.SubContent>
          </ActionMenuRadix.Sub>
        </ActionMenuRadix.Content>
      </ActionMenuRadix.Portal>
    </ActionMenuRadix.Root>
  );
};
