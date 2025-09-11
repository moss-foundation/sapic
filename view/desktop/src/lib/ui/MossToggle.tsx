import { cn } from "@/utils";

interface MossToggleProps {
  checked: boolean;
  onCheckedChange: (checked: boolean) => void;
  labelLeft?: string;
  labelRight?: string;
}

export const MossToggle = ({ checked, onCheckedChange, labelLeft, labelRight }: MossToggleProps) => {
  const handleCheckboxChange = () => {
    onCheckedChange(!checked);
  };

  return (
    <label className="flex w-max cursor-pointer items-center gap-2 select-none">
      {labelLeft && <div>{labelLeft}</div>}

      <div
        className={cn(
          "relative grid h-6 w-12 grid-cols-2 place-items-center rounded-md border transition-colors peer-focus-visible:outline-2 peer-focus-visible:outline-offset-2 peer-focus-visible:outline-(--moss-primary)",
          {
            "background-(--moss-mossToggle-bg) border-(--moss-mossToggle-border)": !checked,
            "background-(--moss-primary) border-(--moss-primary)": checked,
          }
        )}
      >
        <input type="checkbox" checked={checked} onChange={handleCheckboxChange} className="peer sr-only" />

        <div className="background-(--moss-mossToggle-indicator-checked) h-[10px] w-[2px]" />
        <div className="col-start-2 size-2.5 rounded-full border border-(--moss-mossToggle-indicator) bg-transparent" />

        <div
          className={cn(
            "background-(--moss-mossToggle-thumb) absolute top-0 h-full w-1/2 rounded-md border transition-[left]",
            {
              "left-0 border-(--moss-mossToggle-thumb-border)": !checked,
              "left-[50%] border-(--moss-primary)": checked,
            }
          )}
        />
      </div>

      {labelRight && <div>{labelRight}</div>}
    </label>
  );
};
