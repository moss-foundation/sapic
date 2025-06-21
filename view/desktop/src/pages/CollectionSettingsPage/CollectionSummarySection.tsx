import { SectionTitle } from "./SectionTitle";

interface CollectionSummaryProps {
  createdDate?: string;
  requestCount?: number;
}

export const CollectionSummarySection = ({ createdDate = "24.05.2025", requestCount = 4 }: CollectionSummaryProps) => {
  return (
    <div className="text-(--moss-primary-text)">
      <SectionTitle>Summary</SectionTitle>
      <div className="space-y-2">
        <div className="flex items-center gap-2">
          <span className="text-sm">📅 Created:</span>
          <span className="text-sm text-(--moss-secondary-text)">{createdDate}</span>
        </div>
        <div className="flex items-center gap-2">
          <span className="text-sm">🔢</span>
          <span className="text-sm text-(--moss-secondary-text)">{requestCount} requests</span>
        </div>
      </div>
    </div>
  );
};
