import { cn } from "@/utils";

export const NodeLabel = ({
  label,
  searchInput,
  className,
}: {
  label: string | number;
  searchInput?: string;
  className?: string;
}) => {
  const renderHighlightedLabel = () => {
    if (!searchInput) return String(label);

    const regex = new RegExp(`(${searchInput})`, "gi");
    const parts = String(label).split(regex);

    return parts.map((part, index) => {
      if (part.toLowerCase() === searchInput.toLowerCase()) {
        return (
          <span key={index} className="bg-sky-600">
            {part}
          </span>
        );
      }
      return <span key={index}>{part}</span>;
    });
  };

  return (
    <span className={cn("w-max overflow-hidden text-ellipsis whitespace-nowrap", className)}>
      {searchInput ? renderHighlightedLabel() : label}
    </span>
  );
};

export default NodeLabel;
