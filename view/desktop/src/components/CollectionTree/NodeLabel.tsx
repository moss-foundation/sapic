import { cn } from "@/utils";

export const NodeLabel = ({
  label,
  searchInput,
  className,
  onClick,
}: {
  label: string | number;
  searchInput?: string;
  className?: string;
  onClick?: () => void;
}) => {
  const renderHighlightedLabel = () => {
    if (!searchInput) return String(label);

    const regex = new RegExp(`(${searchInput})`, "gi");
    const parts = String(label).split(regex);

    return parts.map((part, index) => {
      if (part.toLowerCase() === searchInput.toLowerCase()) {
        return (
          <span key={index} className="background-(--moss-primary)">
            {part}
          </span>
        );
      }
      return <span key={index}>{part}</span>;
    });
  };

  return (
    <span className={cn("w-max overflow-hidden text-ellipsis whitespace-nowrap", className)} onClick={onClick}>
      {searchInput ? renderHighlightedLabel() : label}
    </span>
  );
};

export default NodeLabel;
