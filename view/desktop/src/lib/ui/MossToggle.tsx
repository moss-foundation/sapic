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
        <div className="background-(--moss-gray-12) relative grid h-6 w-12 grid-cols-2 place-items-center rounded-md border border-(--moss-gray-11) peer-focus-visible:outline-2 peer-focus-visible:outline-offset-2 peer-focus-visible:outline-(--moss-primary) focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-(--moss-primary)">
          <input type="checkbox" checked={checked} onChange={handleCheckboxChange} className="peer sr-only" />

          <div className="background-(--moss-gray-6) h-[2px] w-2" />
          <div className="size-2.5 rounded-full border border-(--moss-gray-6) bg-transparent" />

          <div
            className={cn(
              "absolute top-0 h-full w-1/2 rounded-md border border-(--moss-gray-8) bg-white transition-all",
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
