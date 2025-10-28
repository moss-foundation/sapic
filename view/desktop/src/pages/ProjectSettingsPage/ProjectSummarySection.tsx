import { Icon } from "@/lib/ui";

interface ProjectSummaryProps {
  createdDate?: string;
  requestCount?: number;
}

export const ProjectSummarySection = ({ createdDate = "24.05.2025", requestCount = 4 }: ProjectSummaryProps) => {
  return (
    <div className="text-(--moss-primary-foreground) text-base font-medium">
      <div className="mb-2.5">
        <h3 className="text-(--moss-secondary-foreground) text-sm">Summary</h3>
      </div>
      <div className="space-y-2">
        <div className="flex items-center gap-1">
          <Icon icon="Calendar" className="size-4" />
          <span>Created:</span>
          <span>{createdDate}</span>
        </div>
        <div className="flex items-center gap-1">
          <Icon icon="Requests" className="size-4" />
          <span>{requestCount} requests</span>
        </div>
      </div>
    </div>
  );
};
