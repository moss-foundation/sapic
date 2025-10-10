import ButtonNeutralOutlined from "@/components/ButtonNeutralOutlined";
import ButtonPrimary from "@/components/ButtonPrimary";
import CheckboxWithLabel from "@/components/CheckboxWithLabel";

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
    <div className="flex items-center justify-between border-t border-(--moss-border-color) px-6 py-4">
      <CheckboxWithLabel
        label="Use as default account"
        checked={useAsDefault}
        onCheckedChange={(check) => {
          if (check !== "indeterminate") setUseAsDefault(check);
        }}
      />
      <div className="flex gap-3">
        <ButtonNeutralOutlined type="button" onClick={handleCancel} disabled={isSubmitting}>
          Close
        </ButtonNeutralOutlined>
        <ButtonPrimary disabled={isSubmitDisabled} type="submit">
          {isSubmitting ? "Connecting..." : "Log In"}
        </ButtonPrimary>
      </div>
    </div>
  );
};
