import { useState } from "react";

import { ActionButton, ContextMenu, DropdownMenu } from "@/components";
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
        <ExampleContextMenu />
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
    <DropdownMenu.Root>
      <DropdownMenu.Trigger asChild>
        <ActionButton icon="MoreHorizontal" />
      </DropdownMenu.Trigger>

      <DropdownMenu.Portal>
        <DropdownMenu.Content>
          <DropdownMenu.Item label="Item 1" onSelect={() => console.log("Item 1 selected")} />
          <DropdownMenu.Item label="Item 2" onSelect={() => console.log("Item 2 selected")} />

          <DropdownMenu.Separator />

          <DropdownMenu.CheckboxItem label="Check me" checked={isChecked} onCheckedChange={setIsChecked} />

          <DropdownMenu.Separator />

          <DropdownMenu.RadioGroup value={radioValue} onValueChange={setRadioValue}>
            <DropdownMenu.RadioItem checked={radioValue === "option1"} value="option1" label="Option 1" />
            <DropdownMenu.RadioItem checked={radioValue === "option2"} value="option2" label="Option 2" />
          </DropdownMenu.RadioGroup>

          <DropdownMenu.Separator />

          <DropdownMenu.Sub>
            <DropdownMenu.SubTrigger label="Submenu" />
            <DropdownMenu.SubContent>
              <DropdownMenu.Item hideIcon label="Sub Item 1" onSelect={() => console.log("Sub Item 1 selected")} />
              <DropdownMenu.Item hideIcon label="Sub Item 2" onSelect={() => console.log("Sub Item 2 selected")} />
            </DropdownMenu.SubContent>
          </DropdownMenu.Sub>
        </DropdownMenu.Content>
      </DropdownMenu.Portal>
    </DropdownMenu.Root>
  );
};

const ExampleContextMenu = () => {
  const [isChecked, setIsChecked] = useState(false);
  const [radioValue, setRadioValue] = useState("option1");

  return (
    <ContextMenu.Root>
      <ContextMenu.Trigger asChild>
        <ActionButton icon="Import" />
      </ContextMenu.Trigger>

      <ContextMenu.Portal>
        <ContextMenu.Content>
          <ContextMenu.Item label="Item 1" onSelect={() => console.log("Item 1 selected")} />
          <ContextMenu.Item label="Item 2" onSelect={() => console.log("Item 2 selected")} />

          <ContextMenu.Separator />

          <ContextMenu.CheckboxItem label="Check me" checked={isChecked} onCheckedChange={setIsChecked} />

          <ContextMenu.Separator />

          <ContextMenu.RadioGroup value={radioValue} onValueChange={setRadioValue}>
            <ContextMenu.RadioItem checked={radioValue === "option1"} value="option1" label="Option 1" />
            <ContextMenu.RadioItem checked={radioValue === "option2"} value="option2" label="Option 2" />
          </ContextMenu.RadioGroup>

          <ContextMenu.Separator />

          <ContextMenu.Sub>
            <ContextMenu.SubTrigger label="Submenu" />
            <ContextMenu.SubContent>
              <ContextMenu.Item hideIcon label="Sub Item 1" onSelect={() => console.log("Sub Item 1 selected")} />
              <ContextMenu.Item hideIcon label="Sub Item 2" onSelect={() => console.log("Sub Item 2 selected")} />
            </ContextMenu.SubContent>
          </ContextMenu.Sub>
        </ContextMenu.Content>
      </ContextMenu.Portal>
    </ContextMenu.Root>
  );
};
