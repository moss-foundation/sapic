import React from "react";
import * as DropdownMenu from "@radix-ui/react-dropdown-menu";
import { Icon, Icons } from "../Icon";
import { cn } from "@/utils";

// Types
export type MenuItemType = "action" | "submenu" | "separator" | "header" | "section" | "checkable" | "footer" | "radio";

export interface MenuItemProps {
  id: string;
  type: MenuItemType;
  label?: string;
  icon?: Icons | null;
  iconColor?: string;
  shortcut?: string;
  items?: MenuItemProps[];
  disabled?: boolean;
  checked?: boolean;
  count?: number; // For showing counts like "All Configurations 25"
  variant?: "danger" | "success" | "warning" | "info" | "default";
  sectionTitle?: string;
  footerText?: string;
  value?: string;
  alignWithIcons?: boolean;
}

export interface ActionMenuProps {
  items: MenuItemProps[];
  trigger?: React.ReactNode;
  open: boolean;
  onOpenChange: (open: boolean) => void;
  height?: number;
  align?: "start" | "center" | "end";
  side?: "top" | "right" | "bottom" | "left";
  sideOffset?: number;
  onSelect?: (item: MenuItemProps) => void;
  className?: string;
  type?: "default" | "dropdown";
  selectedValue?: string;
  placeholder?: string;
}

// Styled components using Radix UI primitives
const MenuContent = React.forwardRef<
  React.ElementRef<typeof DropdownMenu.Content>,
  React.ComponentPropsWithoutRef<typeof DropdownMenu.Content> & { height?: number }
>(({ className, height, ...props }, ref) => (
  <DropdownMenu.Content
    ref={ref}
    align="start"
    sideOffset={5}
    className={cn(
      "border-(solid 1 --moss-border-primary) z-50 max-h-[553px] max-w-[293px] min-w-[220px] rounded-md bg-(--moss-primary-background) p-1 shadow-md",
      className
    )}
    style={{
      height,
    }}
    {...props}
  />
));
MenuContent.displayName = "MenuContent";

const MenuItem = React.forwardRef<
  React.ElementRef<typeof DropdownMenu.Item>,
  React.ComponentPropsWithoutRef<typeof DropdownMenu.Item> & {
    variant?: string;
    height?: number;
    hasIcon?: boolean;
    alignWithIcons?: boolean;
  }
>(({ className, variant = "default", height, hasIcon, alignWithIcons, ...props }, ref) => (
  <DropdownMenu.Item
    ref={ref}
    className={cn(
      "relative flex cursor-default items-center rounded-sm px-3 py-1.5 outline-none select-none data-[disabled]:pointer-events-none data-[disabled]:opacity-50 data-[highlighted]:bg-(--moss-secondary-background-hover)",
      {
        "text-green-500": variant === "success",
        "text-red-500": variant === "danger",
        "text-yellow-500": variant === "warning",
        "text-blue-500": variant === "info",
        "h-6": true,
      },
      className
    )}
    style={{ height: "24px" }}
    {...props}
  />
));
MenuItem.displayName = "MenuItem";

const MenuSeparator = React.forwardRef<
  React.ElementRef<typeof DropdownMenu.Separator>,
  React.ComponentPropsWithoutRef<typeof DropdownMenu.Separator>
>(({ className, ...props }, ref) => (
  <DropdownMenu.Separator
    ref={ref}
    className={cn("background-(--moss-border-color) my-1 h-px", className)}
    {...props}
  />
));
MenuSeparator.displayName = "MenuSeparator";

const MenuSubTrigger = React.forwardRef<
  React.ElementRef<typeof DropdownMenu.SubTrigger>,
  React.ComponentPropsWithoutRef<typeof DropdownMenu.SubTrigger> & { hideChevron?: boolean; height?: number }
>(({ className, children, hideChevron, height, ...props }, ref) => (
  <DropdownMenu.SubTrigger
    ref={ref}
    className={cn(
      "flex cursor-default items-center rounded-sm px-3 py-1.5 outline-none select-none data-[disabled]:opacity-50 data-[highlighted]:bg-(--moss-secondary-background-hover) data-[state=open]:bg-(--moss-secondary-background) data-[state=open]:data-[highlighted]:bg-(--moss-secondary-background-hover)",
      {
        "h-6": true,
      },
      className
    )}
    style={{ height: "24px" }}
    {...props}
  >
    {children}
    {!hideChevron && <Icon icon="TreeChevronRight" className="ml-2 h-4 w-4" />}
  </DropdownMenu.SubTrigger>
));
MenuSubTrigger.displayName = "MenuSubTrigger";

const MenuSubContent = React.forwardRef<
  React.ElementRef<typeof DropdownMenu.SubContent>,
  React.ComponentPropsWithoutRef<typeof DropdownMenu.SubContent> & {
    height?: number;
  }
>(({ className, height, ...props }, ref) => (
  <DropdownMenu.SubContent
    ref={ref}
    className={cn(
      "border-(solid 1 --moss-border-primary) z-50 max-h-[553px] max-w-[293px] min-w-[220px] rounded-md bg-(--moss-primary-background) p-1 shadow-md",
      className
    )}
    style={{
      height,
    }}
    {...props}
  />
));
MenuSubContent.displayName = "MenuSubContent";

const MenuLabel = React.forwardRef<
  React.ElementRef<typeof DropdownMenu.Label>,
  React.ComponentPropsWithoutRef<typeof DropdownMenu.Label>
>(({ className, ...props }, ref) => (
  <DropdownMenu.Label
    ref={ref}
    className={cn("px-3 py-2 text-center text-xs font-medium text-(--moss-text-primary)", className)}
    {...props}
  />
));
MenuLabel.displayName = "MenuLabel";

const MenuSectionLabel = React.forwardRef<
  React.ElementRef<typeof DropdownMenu.Label>,
  React.ComponentPropsWithoutRef<typeof DropdownMenu.Label>
>(({ className, ...props }, ref) => (
  <DropdownMenu.Label
    ref={ref}
    className={cn("px-3 py-1 text-xs font-medium text-(--moss-text-secondary)", className)}
    {...props}
  />
));
MenuSectionLabel.displayName = "MenuSectionLabel";

const MenuFooter = React.forwardRef<
  React.ElementRef<typeof DropdownMenu.Label>,
  React.ComponentPropsWithoutRef<typeof DropdownMenu.Label>
>(({ className, ...props }, ref) => (
  <DropdownMenu.Label
    ref={ref}
    className={cn(
      "mt-1 border-t border-(--moss-border-primary) px-3 py-2 text-center text-xs text-(--moss-text-secondary)",
      className
    )}
    {...props}
  />
));
MenuFooter.displayName = "MenuFooter";

const MenuCheckboxItem = React.forwardRef<
  React.ElementRef<typeof DropdownMenu.CheckboxItem>,
  React.ComponentPropsWithoutRef<typeof DropdownMenu.CheckboxItem> & { height?: number; alignWithIcons?: boolean }
>(({ className, children, checked, height, alignWithIcons, ...props }, ref) => (
  <DropdownMenu.CheckboxItem
    ref={ref}
    className={cn(
      "relative flex cursor-default items-center rounded-sm px-3 py-1.5 outline-none select-none data-[disabled]:pointer-events-none data-[disabled]:opacity-50 data-[highlighted]:bg-(--moss-secondary-background-hover)",
      {
        "h-6": true,
      },
      className
    )}
    style={{ height: "24px" }}
    checked={checked}
    {...props}
  >
    <div className="mr-2 flex h-5 w-5 items-center justify-center">
      <DropdownMenu.ItemIndicator>
        <Icon icon="CheckboxIndicator" className="h-4 w-4" />
      </DropdownMenu.ItemIndicator>
    </div>
    {children}
  </DropdownMenu.CheckboxItem>
));
MenuCheckboxItem.displayName = "MenuCheckboxItem";

const MenuRadioItem = React.forwardRef<
  React.ElementRef<typeof DropdownMenu.RadioItem>,
  React.ComponentPropsWithoutRef<typeof DropdownMenu.RadioItem> & { height?: number }
>(({ className, children, height, ...props }, ref) => (
  <DropdownMenu.RadioItem
    ref={ref}
    className={cn(
      "relative flex cursor-default items-center rounded-sm px-3 py-1.5 outline-none select-none data-[disabled]:pointer-events-none data-[highlighted]:bg-(--moss-secondary-background-hover) data-[state=checked]:bg-(--moss-secondary-background)",
      {
        "h-6": true,
      },
      className
    )}
    style={{ height: "24px" }}
    {...props}
  >
    {children}
  </DropdownMenu.RadioItem>
));
MenuRadioItem.displayName = "MenuRadioItem";

// Custom Dropdown Trigger for the dropdown select mode
const DropdownTrigger = React.forwardRef<
  HTMLButtonElement,
  React.ButtonHTMLAttributes<HTMLButtonElement> & {
    placeholder?: string;
    value?: string;
    open?: boolean;
  }
>(({ className, placeholder, value, open, ...props }, ref) => (
  <button
    ref={ref}
    className={cn(
      "flex h-10 w-full items-center justify-between rounded-md border border-(--moss-border-primary) bg-(--moss-primary-background) px-3 py-2 text-(--moss-text-primary) hover:bg-(--moss-secondary-background-hover)",
      className
    )}
    {...props}
  >
    <span>{value || placeholder || "Select..."}</span>
    <Icon icon={open ? "ChevronUp" : "ChevronDown"} className="h-4 w-4" />
  </button>
));
DropdownTrigger.displayName = "DropdownTrigger";

export const ActionMenu: React.FC<ActionMenuProps> = ({
  items,
  trigger,
  open,
  onOpenChange,
  height,
  align = "start",
  side = "bottom",
  sideOffset = 5,
  onSelect,
  className,
  type = "default",
  selectedValue,
  placeholder,
}) => {
  // Find footer items if any
  const footerItems = items.filter((item) => item.type === "footer");
  const normalItems = items.filter((item) => item.type !== "footer");

  // Handler for item selection
  const handleSelect = (item: MenuItemProps) => {
    if (onSelect && item.type !== "separator" && item.type !== "header" && item.type !== "section" && !item.disabled) {
      onSelect(item);
    }
  };

  // Create a custom trigger for dropdown mode
  const dropdownTrigger =
    type === "dropdown" && !trigger ? (
      <DropdownTrigger
        placeholder={placeholder}
        value={selectedValue || items.find((item) => item.value === selectedValue)?.label}
        open={open}
      />
    ) : (
      trigger
    );

  // Recursive function to render menu items
  const renderMenuItems = (menuItems: MenuItemProps[]) => {
    if (menuItems.some((item) => item.type === "radio")) {
      return (
        <DropdownMenu.RadioGroup value={selectedValue}>
          {menuItems.map((item) => {
            if (item.type === "separator") {
              return <MenuSeparator key={item.id} />;
            }

            if (item.type === "header") {
              return <MenuLabel key={item.id}>{item.label}</MenuLabel>;
            }

            if (item.type === "section") {
              return <MenuSectionLabel key={item.id}>{item.sectionTitle}</MenuSectionLabel>;
            }

            if (item.type === "radio") {
              return (
                <MenuRadioItem
                  key={item.id}
                  value={item.value || item.id}
                  disabled={item.disabled}
                  onSelect={() => handleSelect(item)}
                >
                  {item.icon && (
                    <div className="mr-2 flex h-5 w-5 items-center justify-center">
                      <Icon
                        icon={item.icon}
                        className={cn("text-(--moss-icon-primary-text)", item.iconColor && `text-[${item.iconColor}]`)}
                      />
                    </div>
                  )}
                  {!item.icon && item.alignWithIcons && (
                    <div className="mr-2 flex h-5 w-5 items-center justify-center"></div>
                  )}
                  <span className="flex-grow">{item.label}</span>
                  {item.shortcut && <span className="ml-4 text-xs text-(--moss-text-secondary)">{item.shortcut}</span>}
                </MenuRadioItem>
              );
            }

            return renderMenuItem(item);
          })}
        </DropdownMenu.RadioGroup>
      );
    }

    return menuItems.map(renderMenuItem);
  };

  // Render an individual menu item
  const renderMenuItem = (item: MenuItemProps) => {
    if (item.type === "separator") return <MenuSeparator key={item.id} />;
    if (item.type === "header") return <MenuLabel key={item.id}>{item.label}</MenuLabel>;
    if (item.type === "section") return <MenuSectionLabel key={item.id}>{item.sectionTitle}</MenuSectionLabel>;

    if (item.type === "checkable") {
      return (
        <MenuCheckboxItem
          key={item.id}
          checked={item.checked}
          disabled={item.disabled}
          onSelect={() => handleSelect(item)}
        >
          <div className="mr-2 flex h-5 w-5 items-center justify-center">
            <DropdownMenu.ItemIndicator>
              <Icon icon="CheckboxIndicator" className="h-4 w-4" />
            </DropdownMenu.ItemIndicator>
          </div>
          {item.icon && (
            <div className="mr-2 flex h-5 w-5 items-center justify-center">
              <Icon
                icon={item.icon}
                className={cn("text-(--moss-icon-primary-text)", item.iconColor && `text-[${item.iconColor}]`)}
              />
            </div>
          )}
          {!item.icon && item.alignWithIcons && <div className="mr-2 flex h-5 w-5 items-center justify-center"></div>}
          <span className="flex-grow">{item.label}</span>
          {item.count !== undefined && <span className="ml-2 text-xs text-(--moss-text-secondary)">{item.count}</span>}
          {item.shortcut && <span className="ml-4 text-xs text-(--moss-text-secondary)">{item.shortcut}</span>}
        </MenuCheckboxItem>
      );
    }

    if (item.type === "submenu" && item.items?.length) {
      return (
        <DropdownMenu.Sub key={item.id}>
          <MenuSubTrigger>
            {item.icon && (
              <div className="mr-2 flex h-5 w-5 items-center justify-center">
                <Icon
                  icon={item.icon}
                  className={cn("text-(--moss-icon-primary-text)", item.iconColor && `text-[${item.iconColor}]`)}
                />
              </div>
            )}
            {!item.icon && item.alignWithIcons && <div className="mr-2 flex h-5 w-5 items-center justify-center"></div>}
            <span className="flex-grow">{item.label}</span>
            {item.count !== undefined && (
              <span className="ml-2 text-xs text-(--moss-text-secondary)">{item.count}</span>
            )}
            {item.shortcut && <span className="ml-4 text-xs text-(--moss-text-secondary)">{item.shortcut}</span>}
          </MenuSubTrigger>
          <DropdownMenu.Portal>
            <MenuSubContent>{renderMenuItems(item.items)}</MenuSubContent>
          </DropdownMenu.Portal>
        </DropdownMenu.Sub>
      );
    }

    if (item.type === "action") {
      return (
        <MenuItem
          key={item.id}
          disabled={item.disabled}
          variant={item.variant}
          hasIcon={!!item.icon}
          alignWithIcons={item.alignWithIcons}
          onSelect={() => handleSelect(item)}
        >
          {item.icon && (
            <div className="mr-2 flex h-5 w-5 items-center justify-center">
              <Icon
                icon={item.icon}
                className={cn(
                  "text-(--moss-icon-primary-text)",
                  item.iconColor === "green" ? "text-green-500" : item.iconColor && `text-[${item.iconColor}]`
                )}
              />
            </div>
          )}
          {!item.icon && item.alignWithIcons && <div className="mr-2 flex h-5 w-5 items-center justify-center"></div>}
          <span className="flex-grow">{item.label}</span>
          {item.count !== undefined && <span className="ml-2 text-xs text-(--moss-text-secondary)">{item.count}</span>}
          {item.shortcut && <span className="ml-4 text-xs text-(--moss-text-secondary)">{item.shortcut}</span>}
        </MenuItem>
      );
    }

    return null;
  };

  // Render footer items
  const renderFooters = () => {
    return footerItems.map((item) => <MenuFooter key={item.id}>{item.footerText}</MenuFooter>);
  };

  return (
    <DropdownMenu.Root open={open} onOpenChange={onOpenChange}>
      {dropdownTrigger && <DropdownMenu.Trigger asChild>{dropdownTrigger}</DropdownMenu.Trigger>}
      <DropdownMenu.Portal>
        <MenuContent height={height} className={className} align={align} side={side} sideOffset={sideOffset}>
          {renderMenuItems(normalItems)}
          {footerItems.length > 0 && renderFooters()}
        </MenuContent>
      </DropdownMenu.Portal>
    </DropdownMenu.Root>
  );
};

export default ActionMenu;
