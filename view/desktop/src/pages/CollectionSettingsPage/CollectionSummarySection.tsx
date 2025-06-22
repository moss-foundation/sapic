import { Icon } from "@/lib/ui";

interface CollectionSummaryProps {
  createdDate?: string;
  requestCount?: number;
}

export const CollectionSummarySection = ({ createdDate = "24.05.2025", requestCount = 4 }: CollectionSummaryProps) => {
  return (
    <div className="text-sm text-(--moss-primary-text)">
      <div className="mb-2.5">
        <h3 className="font-medium text-(--moss-secondary-text)">Summary</h3>
      </div>
      <div className="space-y-2">
        <div className="flex items-center gap-2">
          <Icon icon="Calendar" className="size-4" />
          <span>Created:</span>
          <span>{createdDate}</span>
        </div>
        <div className="flex items-center gap-2">
          <Icon icon="Requests" className="size-4" />
          <span>{requestCount} requests</span>
        </div>
      </div>
    </div>
  );
};
