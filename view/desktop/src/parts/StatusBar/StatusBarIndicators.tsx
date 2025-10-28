import { Icon } from "@/lib/ui";

export const StatusBarIndicators = () => {
  return (
    <div className="flex h-full items-center">
      <button className="group flex h-full items-center">
        <div className="flex items-center rounded transition">
          <div className="hover:background-(--moss-statusBarItem-background-hover) flex h-[22px] items-center space-x-2 rounded px-1">
            <div className="flex items-center gap-1">
              <Icon className="size-[14px] text-[#E55765]" icon="Failed" />
              <span className="text-(--moss-statusBarItem-foreground) text-sm">2</span>
            </div>
            <div className="flex items-center gap-1">
              <Icon className="size-[14px] text-[#FFAF0F]" icon="Warning" />
              <span className="text-(--moss-statusBarItem-foreground) text-sm">5</span>
            </div>
          </div>
        </div>
      </button>
    </div>
  );
};
