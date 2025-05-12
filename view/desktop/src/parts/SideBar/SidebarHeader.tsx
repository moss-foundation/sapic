import { useState } from "react";

import { ActionButton, ActionMenu } from "@/components";
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
    <ActionMenu.Root>
      <ActionMenu.Trigger asChild>
        <ActionButton icon="MoreHorizontal" />
      </ActionMenu.Trigger>

      <ActionMenu.Portal>
        <ActionMenu.Content>
          <ActionMenu.Item onSelect={() => console.log("Item 1 selected")}>Item 1</ActionMenu.Item>
          <ActionMenu.Item onSelect={() => console.log("Item 2 selected")}>Item 2</ActionMenu.Item>

          <ActionMenu.Separator />

          <ActionMenu.CheckboxItem checked={isChecked} onCheckedChange={setIsChecked}>
            Check me
          </ActionMenu.CheckboxItem>

          <ActionMenu.Separator />

          <ActionMenu.RadioGroup value={radioValue} onValueChange={setRadioValue}>
            <ActionMenu.RadioItem checked={radioValue === "option1"} value="option1">
              Option 1
            </ActionMenu.RadioItem>
            <ActionMenu.RadioItem checked={radioValue === "option2"} value="option2">
              Option 2
            </ActionMenu.RadioItem>
          </ActionMenu.RadioGroup>

          <ActionMenu.Separator />

          <ActionMenu.Sub>
            <ActionMenu.SubTrigger>Submenu</ActionMenu.SubTrigger>
            <ActionMenu.SubContent>
              <ActionMenu.Item hideIcon onSelect={() => console.log("Sub Item 1 selected")}>
                Sub Item 1
              </ActionMenu.Item>
              <ActionMenu.Item hideIcon onSelect={() => console.log("Sub Item 2 selected")}>
                Sub Item 2
              </ActionMenu.Item>
            </ActionMenu.SubContent>
          </ActionMenu.Sub>
        </ActionMenu.Content>
      </ActionMenu.Portal>
    </ActionMenu.Root>
  );
};

const ExampleContextMenuRadix = () => {
  const [isChecked, setIsChecked] = useState(false);
  const [radioValue, setRadioValue] = useState("option1");

  return (
    <ActionMenu.Root>
      <ActionMenu.Trigger asChild openOnRightClick>
        <ActionButton icon="Import" />
      </ActionMenu.Trigger>

      <ActionMenu.Portal>
        <ActionMenu.Content>
          <ActionMenu.Item onSelect={() => console.log("Item 1 selected")}>Item 1</ActionMenu.Item>
          <ActionMenu.Item onSelect={() => console.log("Item 2 selected")}>Item 2</ActionMenu.Item>

          <ActionMenu.Separator />

          <ActionMenu.CheckboxItem checked={isChecked} onCheckedChange={setIsChecked}>
            Check me
          </ActionMenu.CheckboxItem>

          <ActionMenu.Separator />

          <ActionMenu.RadioGroup value={radioValue} onValueChange={setRadioValue}>
            <ActionMenu.RadioItem checked={radioValue === "option1"} value="option1">
              Option 1
            </ActionMenu.RadioItem>
            <ActionMenu.RadioItem checked={radioValue === "option2"} value="option2">
              Option 2
            </ActionMenu.RadioItem>
          </ActionMenu.RadioGroup>

          <ActionMenu.Separator />

          <ActionMenu.Sub>
            <ActionMenu.SubTrigger>Submenu</ActionMenu.SubTrigger>
            <ActionMenu.SubContent>
              <ActionMenu.Item hideIcon onSelect={() => console.log("Sub Item 1 selected")}>
                Sub Item 1
              </ActionMenu.Item>
              <ActionMenu.Item hideIcon onSelect={() => console.log("Sub Item 2 selected")}>
                Sub Item 2
              </ActionMenu.Item>
            </ActionMenu.SubContent>
          </ActionMenu.Sub>
        </ActionMenu.Content>
      </ActionMenu.Portal>
    </ActionMenu.Root>
  );
};
