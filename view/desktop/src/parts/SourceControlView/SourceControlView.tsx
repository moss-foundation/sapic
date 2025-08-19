import { ButtonNeutralOutlined } from "@/components";
import { ButtonPrimary } from "@/components/ButtonPrimary";
import { Textarea } from "@/components/Textarea";

import { SourceControlViewHeader } from "./SourceControlViewHeader";

export const SourceControlView = () => {
  return (
    <div className="flex h-full flex-col">
      <SourceControlViewHeader />
      <div className="flex flex-col gap-2 px-2 py-1">
        <Textarea />

        <div className="flex justify-end gap-2">
          <ButtonNeutralOutlined className="px-3 py-1.5">Commit</ButtonNeutralOutlined>
          <ButtonPrimary className="px-3 py-1.5">Commit and Push</ButtonPrimary>
        </div>
      </div>
    </div>
  );
};
