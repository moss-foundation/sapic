import React from "react";
import * as DropdownMenu from "@radix-ui/react-dropdown-menu";
import { Icon, Icons } from "@/components/Icon";
import { cn } from "@/utils";
import { cva } from "class-variance-authority";

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
  align?: "start" | "center" | "end";
  side?: "top" | "right" | "bottom" | "left";
  sideOffset?: number;
  onSelect?: (item: MenuItemProps) => void;
  className?: string;
  type?: "default" | "dropdown";
  selectedValue?: string;
  placeholder?: string;
}

// Shared menu content styles
const menuContentStyles = cva(
  "border-(solid 1 --moss-border-primary) z-50 max-h-[35rem] max-w-72 min-w-56 rounded-md bg-(--moss-primary-background) p-1 pb-2 shadow-md"
);

// Shared menu item styles
const menuItemStyles = cva(
  "relative flex cursor-default items-center rounded-sm px-3 py-1.5 outline-none select-none data-[disabled]:pointer-events-none data-[disabled]:opacity-50",
  {
    variants: {
      variant: {
        default: "",
        success: "text-green-500",
        danger: "text-red-500",
        warning: "text-yellow-500",
        info: "text-blue-500",
      },
      highlighted: {
        true: "data-[highlighted]:bg-(--moss-info-background-hover)",
      },
      state: {
        normal: "",
        checked: "data-[state=checked]:bg-(--moss-secondary-background)",
        open: "data-[state=open]:bg-(--moss-secondary-background) data-[state=open]:data-[highlighted]:bg-(--moss-info-background-hover)",
      },
    },
    defaultVariants: {
      variant: "default",
      highlighted: true,
      state: "normal",
    },
  }
);

const labelStyles = "truncate max-w-[200px] text-(--moss-primary-text)";

const MenuContent = React.forwardRef<
  React.ElementRef<typeof DropdownMenu.Content>,
  React.ComponentPropsWithoutRef<typeof DropdownMenu.Content>
>(({ className, ...props }, ref) => (
  <DropdownMenu.Content
    ref={ref}
    align="start"
    sideOffset={5}
    className={cn(menuContentStyles(), className)}
    {...props}
  />
));
MenuContent.displayName = "MenuContent";

const MenuItem = React.forwardRef<
  React.ElementRef<typeof DropdownMenu.Item>,
  React.ComponentPropsWithoutRef<typeof DropdownMenu.Item> & {
    variant?: string;
    hasIcon?: boolean;
    alignWithIcons?: boolean;
  }
>(({ className, variant = "default", hasIcon, alignWithIcons, ...props }, ref) => (
  <DropdownMenu.Item
    ref={ref}
    className={cn(menuItemStyles({ variant: variant as any, highlighted: true }), "h-6", className)}
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
  React.ComponentPropsWithoutRef<typeof DropdownMenu.SubTrigger> & { hideChevron?: boolean }
>(({ className, children, hideChevron, ...props }, ref) => (
  <DropdownMenu.SubTrigger
    ref={ref}
    className={cn(menuItemStyles({ highlighted: true, state: "open" }), "h-6", className)}
    {...props}
  >
    {children}
    {!hideChevron && <Icon icon="TreeChevronRight" className="ml-2 h-4 w-4 text-(--moss-not-selected-item-color)" />}
  </DropdownMenu.SubTrigger>
));
MenuSubTrigger.displayName = "MenuSubTrigger";

const MenuSubContent = React.forwardRef<
  React.ElementRef<typeof DropdownMenu.SubContent>,
  React.ComponentPropsWithoutRef<typeof DropdownMenu.SubContent>
>(({ className, ...props }, ref) => (
  <DropdownMenu.SubContent ref={ref} className={cn(menuContentStyles(), className)} {...props} />
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
  React.ComponentPropsWithoutRef<typeof DropdownMenu.CheckboxItem> & { alignWithIcons?: boolean }
>(({ className, children, checked, alignWithIcons, ...props }, ref) => (
  <DropdownMenu.CheckboxItem
    ref={ref}
    className={cn(menuItemStyles({ highlighted: true }), "h-6", className)}
    checked={checked}
    {...props}
  >
    <div className="mr-2 flex h-5 w-5 items-center justify-center">
      <DropdownMenu.ItemIndicator>
        <Icon icon="CheckboxIndicator" className="h-4 w-4 text-(--moss-icon-primary-text)" />
      </DropdownMenu.ItemIndicator>
    </div>
    {children}
  </DropdownMenu.CheckboxItem>
));
MenuCheckboxItem.displayName = "MenuCheckboxItem";

const MenuRadioItem = React.forwardRef<
  React.ElementRef<typeof DropdownMenu.RadioItem>,
  React.ComponentPropsWithoutRef<typeof DropdownMenu.RadioItem>
>(({ className, children, ...props }, ref) => (
  <DropdownMenu.RadioItem
    ref={ref}
    className={cn(menuItemStyles({ highlighted: true, state: "checked" }), "h-6", className)}
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
      "flex h-10 w-full items-center justify-between rounded-md border border-(--moss-border-primary) bg-(--moss-primary-background) px-3 py-2 text-(--moss-text-primary) hover:bg-(--moss-info-background-hover)",
      className
    )}
    {...props}
  >
    <span className={labelStyles}>{value || placeholder || "Select..."}</span>
    <Icon icon={open ? "ChevronUp" : "ChevronDown"} className="h-4 w-4 text-(--moss-icon-primary-text)" />
  </button>
));
DropdownTrigger.displayName = "DropdownTrigger";

// Helper component for menu item icons
const MenuItemIcon = ({ icon, iconColor }: { icon?: Icons | null; iconColor?: string }) => {
  if (!icon) return null;

  return (
    <div className="mr-2 flex h-5 w-5 items-center justify-center">
      <Icon
        icon={icon}
        className={cn(
          "text-(--moss-icon-primary-text)",
          iconColor === "green" ? "text-green-500" : iconColor && `text-[${iconColor}]`
        )}
      />
    </div>
  );
};

// Helper component for menu item trailing elements
const MenuItemTrailing = ({ count, shortcut }: { count?: number; shortcut?: string }) => (
  <>
    {count !== undefined && <span className="ml-2 text-xs text-(--moss-text-secondary)">{count}</span>}
    {shortcut && <span className="ml-4 text-xs text-(--moss-not-selected-item-color)">{shortcut}</span>}
  </>
);

export const ActionMenu: React.FC<ActionMenuProps> = ({
  items,
  trigger,
  open,
  onOpenChange,
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
                  <MenuItemIcon icon={item.icon} iconColor={item.iconColor} />
                  {!item.icon && item.alignWithIcons && (
                    <div className="mr-2 flex h-5 w-5 items-center justify-center"></div>
                  )}
                  <span className={cn("flex-grow", labelStyles)}>{item.label}</span>
                  <MenuItemTrailing count={item.count} shortcut={item.shortcut} />
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
          alignWithIcons={item.alignWithIcons}
          onSelect={() => handleSelect(item)}
        >
          <MenuItemIcon icon={item.icon} iconColor={item.iconColor} />
          {!item.icon && item.alignWithIcons && <div className="mr-2 flex h-5 w-5 items-center justify-center"></div>}
          <span className={cn("flex-grow", labelStyles)}>{item.label}</span>
          <MenuItemTrailing count={item.count} shortcut={item.shortcut} />
        </MenuCheckboxItem>
      );
    }

    if (item.type === "submenu" && item.items?.length) {
      return (
        <DropdownMenu.Sub key={item.id}>
          <MenuSubTrigger>
            <MenuItemIcon icon={item.icon} iconColor={item.iconColor} />
            {!item.icon && item.alignWithIcons && <div className="mr-2 flex h-5 w-5 items-center justify-center"></div>}
            <span className={cn("flex-grow", labelStyles)}>{item.label}</span>
            <MenuItemTrailing count={item.count} shortcut={item.shortcut} />
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
          <MenuItemIcon icon={item.icon} iconColor={item.iconColor} />
          {!item.icon && item.alignWithIcons && <div className="mr-2 flex h-5 w-5 items-center justify-center"></div>}
          <span className={cn("flex-grow", labelStyles)}>{item.label}</span>
          <MenuItemTrailing count={item.count} shortcut={item.shortcut} />
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
        <MenuContent className={className} align={align} side={side} sideOffset={sideOffset}>
          {renderMenuItems(normalItems)}
          {footerItems.length > 0 && renderFooters()}
        </MenuContent>
      </DropdownMenu.Portal>
    </DropdownMenu.Root>
  );
};

export default ActionMenu;
