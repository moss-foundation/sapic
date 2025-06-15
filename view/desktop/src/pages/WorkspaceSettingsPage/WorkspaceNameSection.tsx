import { InputOutlined } from "@/components";
import ButtonNeutralOutlined from "@/components/ButtonNeutralOutlined";
import ButtonPrimary from "@/components/ButtonPrimary";

interface WorkspaceNameProps {
  name: string;
  setName: (name: string) => void;
  hasChanges: boolean;
  isPending: boolean;
  onSave: () => void;
  onReset: () => void;
  onBlur: () => void;
}

export const WorkspaceNameSection = ({
  name,
  setName,
  hasChanges,
  isPending,
  onSave,
  onReset,
  onBlur,
}: WorkspaceNameProps) => {
  return (
    <div>
      <div className="flex items-start gap-3.5 text-(--moss-primary-text)">
        <label className="mt-1 font-medium">Name:</label>
        <div>
          <InputOutlined
            size="sm"
            value={name}
            onChange={(e) => setName(e.target.value)}
            onBlur={onBlur}
            onKeyDown={(e) => {
              if (e.key === "Enter") {
                e.preventDefault();
                onSave();
              } else if (e.key === "Escape") {
                e.preventDefault();
                onReset();
              }
            }}
            placeholder="Enter workspace name..."
            className="w-72 border-(--moss-input-border)"
          />
          <p className="mt-1 w-72 text-sm text-(--moss-secondary-text)">
            Invalid filename characters (e.g. / \ : * ? " &lt; &gt; |) will be escaped
          </p>
        </div>
      </div>
      {hasChanges && (
        <div className="mt-3 flex items-center gap-2">
          <ButtonPrimary onClick={onSave} disabled={isPending || !name.trim()} size="md">
            {isPending ? "Saving..." : "Save"}
          </ButtonPrimary>
          <ButtonNeutralOutlined onClick={onReset} size="md">
            Cancel
          </ButtonNeutralOutlined>
        </div>
      )}
    </div>
  );
};
