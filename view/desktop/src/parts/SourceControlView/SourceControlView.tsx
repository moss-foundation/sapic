import { Textarea } from "@/components/Textarea";
import { Button } from "@/lib/ui";

import { SourceControlViewHeader } from "./SourceControlViewHeader";

export const SourceControlView = () => {
  return (
    <div className="flex h-full flex-col">
      <SourceControlViewHeader />
      <div className="flex flex-col gap-2 px-2 py-1">
        <Textarea />

        <div className="@container/buttons flex flex-wrap justify-end gap-2">
          <Button intent="outlined" className="w-full px-3 py-1.5 @[210px]/buttons:w-auto">
            Commit
          </Button>
          <Button intent="primary" className="w-full px-3 py-1.5 @[210px]/buttons:w-auto">
            Commit and Push
          </Button>
        </div>
      </div>
    </div>
  );
};
