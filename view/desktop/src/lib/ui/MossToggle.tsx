import { cn } from "@/utils";

interface MossToggleProps {
  checked: boolean;
  onCheckedChange: (checked: boolean) => void;
}

export const MossToggle = ({ checked, onCheckedChange }: MossToggleProps) => {
  const handleCheckboxChange = () => {
    onCheckedChange(!checked);
  };

  return (
    <>
      <label className="flex w-max cursor-pointer items-center select-none">
        <div className="background-(--moss-mossToggle-bg) relative grid h-6 w-12 grid-cols-2 place-items-center rounded-md border border-(--moss-mossToggle-border) peer-focus-visible:outline-2 peer-focus-visible:outline-offset-2 peer-focus-visible:outline-(--moss-primary)">
          <input type="checkbox" checked={checked} onChange={handleCheckboxChange} className="peer sr-only" />

          <div className="background-(--moss-mossToggle-indicator) h-[2px] w-2" />
          <div className="size-2.5 rounded-full border border-(--moss-mossToggle-indicator) bg-transparent" />

          <div
            className={cn(
              "background-(--moss-mossToggle-thumb) absolute top-0 h-full w-1/2 rounded-md border border-(--moss-mossToggle-thumb-border) transition-all",
              {
                "left-0": checked,
                "left-[50%]": !checked,
              }
            )}
          ></div>
        </div>
      </label>
    </>
  );
};
