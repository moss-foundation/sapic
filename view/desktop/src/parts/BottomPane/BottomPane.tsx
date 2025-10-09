import { testLogEntries } from "@/assets/testLogEntries";
import { Scrollbar } from "@/lib/ui";

export const BottomPane = () => {
  return (
    <div className="background-(--moss-primary-background) h-full w-full">
      <Scrollbar className="h-full">
        <div className={`p-2 font-mono text-sm select-none hover:select-text`}>
          <div className="mb-2 font-semibold">Application Logs:</div>

          {/* TODO removing this test data don't forget to remove testLogEntries.ts file  */}
          {testLogEntries.map((log, index) => (
            <div key={index} className="mb-1 flex">
              <span className="mr-2 text-(--moss-secondary-text)">{log.timestamp}</span>
              <span
                className={`mr-2 min-w-16 font-medium ${
                  log.level === "ERROR"
                    ? "text-red-500"
                    : log.level === "WARNING"
                      ? "text-amber-500"
                      : log.level === "DEBUG"
                        ? "text-blue-500"
                        : "text-green-500"
                }`}
              >
                {log.level}
              </span>
              <span className="mr-2 min-w-32 font-semibold">{log.service}:</span>
              <span>{log.message}</span>
            </div>
          ))}
        </div>
      </Scrollbar>
    </div>
  );
};
