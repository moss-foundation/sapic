import { Button } from "@/lib/ui";
import CheckboxWithLabel from "@/lib/ui/CheckboxWithLabel";

import { CREATE_TAB, IMPORT_TAB } from "../constansts";

interface FooterActionsProps {
  openAutomatically: boolean;
  setOpenAutomatically: (openAutomatically: boolean) => void;
  handleCancel: () => void;
  isSubmitDisabled: boolean;
  tab: typeof CREATE_TAB | typeof IMPORT_TAB;
}

export const FooterActions = ({
  openAutomatically,
  setOpenAutomatically,
  handleCancel,
  isSubmitDisabled,
  tab,
}: FooterActionsProps) => {
  return (
    <div className="border-(--moss-border) flex items-center justify-between border-t px-6 py-4">
      <CheckboxWithLabel
        label="Open automatically after creation"
        checked={openAutomatically}
        onCheckedChange={(check) => {
          if (check !== "indeterminate") setOpenAutomatically(check);
        }}
      />
      <div className="flex gap-3">
        <Button intent="outlined" type="button" onClick={handleCancel}>
          Close
        </Button>
        <Button intent="primary" disabled={isSubmitDisabled} type="submit">
          {tab === CREATE_TAB ? "Create" : "Import"}
        </Button>
      </div>
    </div>
  );
};
