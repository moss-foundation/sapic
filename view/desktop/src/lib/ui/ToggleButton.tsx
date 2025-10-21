import { cn } from "@/utils";

interface ToggleButtonProps {
  checked: boolean;
  onCheckedChange: (checked: boolean) => void;
  labelLeft?: string;
  labelRight?: string;
  disabled?: boolean;
}

export const ToggleButton = ({
  checked,
  onCheckedChange,
  labelLeft,
  labelRight,
  disabled = false,
}: ToggleButtonProps) => {
  const handleCheckboxChange = () => {
    onCheckedChange(!checked);
  };

  return (
    <label className="flex w-max cursor-pointer items-center gap-2 select-none">
      {labelLeft && <div>{labelLeft}</div>}

      <div
        className={cn(
          "relative grid h-6 w-12 grid-cols-2 place-items-center rounded-md border outline-2 outline-offset-2 outline-transparent transition-colors has-focus-visible:outline-2 has-focus-visible:outline-offset-2 has-focus-visible:outline-(--moss-accent)",
          {
            "background-(--moss-toggleButton-background) border-(--moss-toggleButton-border)": !checked && !disabled,
            "background-(--moss-accent) border-(--moss-accent)": checked && !disabled,
            "background-(--moss-background-disabled) cursor-not-allowed border-(--moss-border-disabled)": disabled,
          }
        )}
      >
        <input
          type="checkbox"
          checked={checked}
          onChange={handleCheckboxChange}
          className="sr-only"
          disabled={disabled}
        />

        <div
          className={cn("h-[10px] w-[2px]", {
            "background-(--moss-foreground-disabled)": disabled,
            "background-(--moss-toggleButton-indicator-checked)": !disabled,
          })}
        />
        <div
          className={cn("size-2.5 rounded-full border border-(--moss-toggleButton-indicator) bg-transparent", {
            "border-(--moss-foreground-disabled)": disabled,
          })}
        />

        <div
          className={cn(
            "background-(--moss-toggleButton-thumb) absolute top-0 h-full w-1/2 rounded-md border transition-[left]",
            {
              "left-0": !checked,
              "left-[50%]": checked,
              "border-(--moss-toggleButton-thumb-border)": !checked && !disabled,
              "border-(--moss-accent)": checked && !disabled,
              "border-(--moss-border-disabled)": disabled,
            }
          )}
        />
      </div>

      {labelRight && <div>{labelRight}</div>}
    </label>
  );
};
