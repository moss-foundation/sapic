import { cn } from "@/utils";
import { EntryKind, EntryProtocol } from "@repo/moss-collection";

// TODO: Remove this when we have real icons for the collections
export const DebugCollectionIconPlaceholder = ({
  protocol,
  type,
  className,
}: {
  protocol: EntryProtocol | undefined;
  type: EntryKind;
  className?: string;
}) => {
  if (type === "Dir")
    return (
      <svg
        className={cn("min-h-4 min-w-4", className)}
        width="16"
        height="16"
        viewBox="0 0 16 16"
        fill="none"
        xmlns="http://www.w3.org/2000/svg"
      >
        <path
          d="M8.10584 4.34613L8.25344 4.5H8.46667H13C13.8284 4.5 14.5 5.17157 14.5 6V12.1333C14.5 12.9529 13.932 13.5 13.3667 13.5H2.63333C2.06804 13.5 1.5 12.9529 1.5 12.1333V3.86667C1.5 3.04707 2.06804 2.5 2.63333 2.5H6.1217C6.25792 2.5 6.38824 2.55557 6.48253 2.65387L8.10584 4.34613Z"
          fill="#EBECF0"
          stroke="#6C707E"
        />
      </svg>
    );

  let color = "";

  switch (protocol) {
    case "Get":
      color = "text-blue-500";
      break;
    case "Post":
      color = "text-green-500";
      break;
    case "Put":
      color = "text-yellow-500";
      break;
    case "Delete":
      color = "text-red-500";
      break;
    case "WebSocket":
      color = "text-purple-500";
      break;
    case "Graphql":
      color = "text-pink-500";
      break;
    case "Grpc":
      color = "text-indigo-500";
      break;
    default:
      color = "text-gray-500";
      break;
  }

  return <div className={cn("text-sm lowercase", color, className)}>{protocol}</div>;
};
