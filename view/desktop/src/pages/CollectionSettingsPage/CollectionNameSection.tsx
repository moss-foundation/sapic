import { InputOutlined } from "@/components";

interface CollectionNameProps {
  name: string;
  setName: (name: string) => void;
  repository: string;
  setRepository: (repository: string) => void;
  onBlur: () => void;
  onSave: () => void;
}

export const CollectionNameSection = ({
  name,
  setName,
  repository,
  setRepository,
  onBlur,
  onSave,
}: CollectionNameProps) => {
  return (
    <div className="space-y-6">
      {/* Name Field */}
      <div className="flex items-start gap-3.5 text-(--moss-primary-text)">
        <label className="mt-1 w-20 font-medium">Name:</label>
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
            placeholder="Enter collection name..."
            className="w-72 border-(--moss-input-border)"
          />
          <p className="mt-1 w-72 text-sm text-(--moss-secondary-text)">
            Invalid filename characters (e.g. / \ : * ? " &lt; &gt; |) will be escaped
          </p>
        </div>
      </div>

      {/* Repository Field */}
      <div className="flex items-start gap-3.5 text-(--moss-primary-text)">
        <label className="mt-1 w-20 font-medium">Repository:</label>
        <div>
          <InputOutlined
            size="sm"
            value={repository}
            onChange={(e) => setRepository(e.target.value)}
            onBlur={onBlur}
            onKeyDown={(e) => {
              if (e.key === "Enter") {
                e.preventDefault();
                onSave();
              }
            }}
            placeholder="Enter repository URL..."
            className="w-72 border-(--moss-input-border)"
          />
        </div>
      </div>
    </div>
  );
};
