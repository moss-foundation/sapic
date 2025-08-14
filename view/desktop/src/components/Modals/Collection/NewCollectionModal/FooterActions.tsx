import { ButtonNeutralOutlined, ButtonPrimary } from "@/components";
import CheckboxWithLabel from "@/components/CheckboxWithLabel";

interface FooterActionsProps {
  openAutomatically: boolean;
  setOpenAutomatically: (openAutomatically: boolean) => void;
  handleCancel: () => void;
  isSubmitDisabled: boolean;
}

export const FooterActions = ({
  openAutomatically,
  setOpenAutomatically,
  handleCancel,
  isSubmitDisabled,
}: FooterActionsProps) => {
  return (
    <div className="flex items-center justify-between px-6 py-2.5">
      <CheckboxWithLabel
        label="Open automatically after creation"
        checked={openAutomatically}
        onCheckedChange={(check) => {
          if (check !== "indeterminate") setOpenAutomatically(check);
        }}
      />
      <div className="flex gap-3">
        <ButtonNeutralOutlined type="button" onClick={handleCancel}>
          Close
        </ButtonNeutralOutlined>
        <ButtonPrimary disabled={isSubmitDisabled} type="submit">
          Create
        </ButtonPrimary>
      </div>
    </div>
  );
};
