import { Row } from "@tanstack/react-table";
import { Icon } from "@/lib/ui";
import { ParameterData } from "@/components/Table";

export const ActionsCell = ({ row }: { row: Row<ParameterData> }) => {
  const isDisabled = row.original.properties.disabled;

  return (
    <div className={`flex items-center gap-0.5 ${isDisabled ? "opacity-40" : ""}`}>
      <button>
        <Icon icon="AddToVcs" className="size-4" />
      </button>
      <button>
        <Icon icon="ConfigMap" className="size-4" />
      </button>
      <button>
        <Icon icon="RemoveCircle" className="size-4" />
      </button>
    </div>
  );
};
