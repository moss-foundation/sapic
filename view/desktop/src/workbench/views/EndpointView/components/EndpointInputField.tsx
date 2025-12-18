import { Button, Icon } from "@/lib/ui";
import { cn } from "@/utils";
import { ActionMenu } from "@/workbench/ui/components";
import { UrlEditor } from "@/workbench/ui/components/UrlEditor/UrlEditor";
import { ResourceProtocol } from "@repo/moss-project";

interface EndpointInputFieldProps {
  className?: string;
  initialProtocol?: string;
  initialUrl?: string;
  onSend?: (method: string, url: string) => void;
  onUrlChange?: (url: string) => void;
  onProtocolChange?: (protocol: ResourceProtocol) => void;
}

const HTTP_METHODS = [
  "Get",
  "Post",
  "Put",
  "Delete",
  "WebSocket",
  "Graphql",
  "Grpc",
] as const satisfies ResourceProtocol[];

export const EndpointInputField = ({
  className,
  initialProtocol = "POST",
  initialUrl,
  onSend,
  onUrlChange,
  onProtocolChange,
}: EndpointInputFieldProps) => {
  const handleUrlChange = (newUrl: string) => {
    onUrlChange?.(newUrl);
  };

  const handleSend = () => {
    onSend?.(initialProtocol, initialUrl ?? "");
  };

  const handleProtocolChange = (newProtocol: ResourceProtocol) => {
    onProtocolChange?.(newProtocol);
  };

  return (
    <div
      className={cn(
        "border-(--moss-border) relative grid grid-cols-[auto_1fr_auto] items-center gap-2 rounded-md border p-0.5",
        className
      )}
    >
      <div className="relative flex h-full items-center">
        <ActionMenu.Root>
          <ActionMenu.Trigger asChild>
            <button
              className={cn(
                "flex h-full items-center justify-between",
                "gap-1.25 px-1",
                "transition-colors",
                "rounded-md",
                "cursor-pointer font-bold",
                "hover:background-(--moss-secondary-background-hover) text-(--moss-orange-5)"
              )}
            >
              <span className="min-w-[50px] text-left uppercase">{initialProtocol}</span>
              <Icon icon="ChevronDown" />
            </button>
          </ActionMenu.Trigger>
          <ActionMenu.Content>
            {HTTP_METHODS.map((httpMethod) => (
              <ActionMenu.Item
                key={httpMethod}
                onClick={() => handleProtocolChange(httpMethod)}
                className={cn(
                  initialProtocol === httpMethod &&
                    "background-(--moss-secondary-background-hover) text-(--moss-controls-foreground) font-medium"
                )}
              >
                <span className="uppercase">{httpMethod}</span>
              </ActionMenu.Item>
            ))}
          </ActionMenu.Content>
        </ActionMenu.Root>
      </div>

      <div className="min-w-0">
        <UrlEditor value={initialUrl} onChange={handleUrlChange} />
      </div>

      <Button intent="primary" onClick={handleSend}>
        Send
      </Button>
    </div>
  );
};
