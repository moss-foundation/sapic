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

export const WorkspaceName = ({
  name,
  setName,
  hasChanges,
  isPending,
  onSave,
  onReset,
  onBlur,
}: WorkspaceNameProps) => {
  return (
    <div className="mt-4">
      <h3 className="mb-2 font-medium text-[var(--moss-select-text-outlined)]">Name:</h3>
      <div className="w-[400px]">
        <InputOutlined
          size="md"
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
        />
      </div>
      <p className="mt-1 text-xs text-gray-500">
        Invalid filename characters (e.g. / \ : * ? " &lt; &gt; |) will be escaped
      </p>
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
