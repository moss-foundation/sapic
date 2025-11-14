import { Button } from "@/lib/ui";
import CheckboxWithLabel from "@/lib/ui/CheckboxWithLabel";

interface FooterActionsProps {
  useAsDefault: boolean;
  setUseAsDefault: (value: boolean) => void;
  handleCancel: () => void;
  isSubmitDisabled: boolean;
  isSubmitting: boolean;
}

export const FooterActions = ({
  useAsDefault,
  setUseAsDefault,
  handleCancel,
  isSubmitDisabled,
  isSubmitting,
}: FooterActionsProps) => {
  return (
    <div className="border-(--moss-border) flex items-center justify-between border-t px-6 py-4">
      <CheckboxWithLabel
        label="Use as default account"
        checked={useAsDefault}
        onCheckedChange={(check) => {
          if (check !== "indeterminate") setUseAsDefault(check);
        }}
      />
      <div className="flex gap-3">
        <Button intent="outlined" type="button" onClick={handleCancel} disabled={isSubmitting}>
          Close
        </Button>
        <Button intent="primary" disabled={isSubmitDisabled} type="submit">
          {isSubmitting ? "Connecting..." : "Log In"}
        </Button>
      </div>
    </div>
  );
};
