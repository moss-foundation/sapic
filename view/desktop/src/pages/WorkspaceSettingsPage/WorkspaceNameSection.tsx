import { InputOutlined } from "@/components";
import { VALID_NAME_PATTERN } from "@/constants/validation";

interface WorkspaceNameProps {
  name: string;
  setName: (name: string) => void;
  onBlur: () => void;
  onSave: () => void;
}

export const WorkspaceNameSection = ({ name, setName, onBlur, onSave }: WorkspaceNameProps) => {
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
              }
            }}
            placeholder="Enter workspace name..."
            pattern={VALID_NAME_PATTERN}
            className="w-72 border-(--moss-input-border)"
          />
          <p className="mt-1 w-72 text-sm text-(--moss-secondary-text)">
            Invalid filename characters (e.g. / \ : * ? " &lt; &gt; |) will be escaped
          </p>
        </div>
      </div>
    </div>
  );
};
