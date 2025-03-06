export const NodeLabel = ({ label, searchInput }: { label: string | number; searchInput?: string }) => {
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
    <span className="text-ellipsis whitespace-nowrap w-max overflow-hidden">
      {searchInput ? renderHighlightedLabel() : label}
    </span>
  );
};

export default NodeLabel;
