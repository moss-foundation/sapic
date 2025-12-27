import { cn } from "@/utils";
import { useTabbedPaneStore } from "@/workbench/store/tabbedPane";
import { PageContent, PageHeader } from "@/workbench/ui/components";
import { PageView } from "@/workbench/ui/components/PageView/PageView";
import Metadata from "@/workbench/ui/parts/TabbedPane/DebugComponents/Metadata";
import { DefaultViewProps } from "@/workbench/ui/parts/TabbedPane/types";

export const DefaultView = ({ ...props }: DefaultViewProps) => {
  const { showDebugPanels } = useTabbedPaneStore();

  return (
    <PageView>
      <PageHeader icon="Placeholder" {...props} />
      <PageContent className={cn("relative", showDebugPanels && "border-2 border-dashed border-orange-500")}>
        <span className="pointer-events-none absolute left-1/2 top-1/2 flex -translate-x-1/2 -translate-y-1/2 transform flex-col opacity-50">
          <span className="text-[42px] leading-[42px]">Default Page</span>
          <span className="text-sm leading-3">This is a placeholder default page</span>
        </span>

        {showDebugPanels && (
          <Metadata
            onClick={() => {
              props.api.setRenderer(props.api.renderer === "always" ? "onlyWhenVisible" : "always");
            }}
            api={props.api}
          />
        )}
      </PageContent>
    </PageView>
  );
};
