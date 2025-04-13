import { Scrollbar } from "@/components/Scrollbar";

const LogsPanel = ({
  logLines,
  onClear,
}: {
  logLines: { text: string; timestamp?: Date; backgroundColor?: string }[];
  onClear: () => void;
}) => {
  return (
    <div className="ml-2 flex w-[400px] shrink-0 flex-col overflow-hidden bg-black font-mono text-white">
      <Scrollbar className="grow overflow-auto">
        {logLines.map((line, i) => (
          <div
            className="flex h-[30px] items-center overflow-hidden text-[13px] text-ellipsis whitespace-nowrap"
            style={{ backgroundColor: line.backgroundColor }}
            key={i}
          >
            <span className="mr-1 flex h-full max-w-[20px] min-w-[20px] items-center border-r border-gray-500 pl-1 text-gray-500">
              {logLines.length - i}
            </span>
            <span>
              {line.timestamp && (
                <span className="px-[2px] text-[0.7em]">{line.timestamp.toISOString().substring(11, 23)}</span>
              )}
              <span>{line.text}</span>
            </span>
          </div>
        ))}
      </Scrollbar>
      <Scrollbar className="flex justify-end p-1">
        <button onClick={onClear}>Clear</button>
      </Scrollbar>
    </div>
  );
};

export default LogsPanel;
