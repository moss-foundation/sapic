import CheckboxWithLabel from "@/components/CheckboxWithLabel";
import { SectionTitle } from "./SectionTitle";

interface WorkspaceStartupProps {
  reopenOnNextSession: boolean;
  setReopenOnNextSession: (value: boolean) => void;
  openPreviousWindows: boolean;
  setOpenPreviousWindows: (value: boolean) => void;
}

export const WorkspaceStartupSection = ({
  reopenOnNextSession,
  setReopenOnNextSession,
  openPreviousWindows,
  setOpenPreviousWindows,
}: WorkspaceStartupProps) => {
  return (
    <div className="mt-6 text-(--moss-primary-text)">
      <SectionTitle>Startup</SectionTitle>
      <div className="space-y-3 pl-5">
        <CheckboxWithLabel
          checked={reopenOnNextSession}
          onCheckedChange={(checked) => setReopenOnNextSession(checked === true)}
          label="Reopen this workspace on next session"
        />
        <CheckboxWithLabel
          checked={openPreviousWindows}
          onCheckedChange={(checked) => setOpenPreviousWindows(checked === true)}
          label="Open previous windows and tabs"
        />
      </div>
    </div>
  );
};
