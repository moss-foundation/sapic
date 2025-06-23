import React from "react";
import { CollectionNameSection } from "../CollectionNameSection";
import { CollectionSummarySection } from "../CollectionSummarySection";
import { CollectionDangerZoneSection } from "../CollectionDangerZoneSection";

interface OverviewTabContentProps {
  name: string;
  setName: (name: string) => void;
  repository: string;
  setRepository: (repository: string) => void;
  onSave: () => void;
  onBlur: () => void;
  onDeleteClick: () => void;
}

export const OverviewTabContent: React.FC<OverviewTabContentProps> = ({
  name,
  setName,
  repository,
  setRepository,
  onSave,
  onBlur,
  onDeleteClick,
}) => {
  return (
    <div className="relative flex h-full min-w-[800px] justify-center">
      {/* Main Content - Centered on full page width */}
      <div className="w-full max-w-2xl space-y-9 px-6 py-5">
        <CollectionNameSection
          name={name}
          setName={setName}
          repository={repository}
          setRepository={setRepository}
          onSave={onSave}
          onBlur={onBlur}
        />

        <CollectionDangerZoneSection onDeleteClick={onDeleteClick} />
      </div>

      {/* Right Column - Summary positioned absolutely on the right */}
      <div className="absolute top-0 right-2 w-60 py-2">
        <CollectionSummarySection />
      </div>
    </div>
  );
};
