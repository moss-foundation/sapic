import { useState } from "react";

import { VALID_NAME_PATTERN } from "@/constants/validation";
import { useGetLocalProjectSummaryById } from "@/db/projectSummaries/hooks/useGetLocalProjectSummaryById";
import { projectService } from "@/domains/project/projectService";
import { useModal } from "@/hooks";
import Input from "@/lib/ui/Input";
import { DeleteProjectModal } from "@/workbench/ui/components/Modals/Project/DeleteProjectModal";

import { ProjectDangerZoneSection } from "../ProjectDangerZoneSection";
import { ProjectSettingsViewProps } from "../ProjectSettingsView";
import { ProjectSummarySection } from "../ProjectSummarySection";

export const OverviewTabContent = ({ params, containerApi }: ProjectSettingsViewProps) => {
  const { data: projectSummary } = useGetLocalProjectSummaryById(params.projectId);

  const [name, setName] = useState(projectSummary?.name || "");
  const [repository, setRepository] = useState("");

  const { showModal, closeModal, openModal } = useModal();

  if (projectSummary) {
    const currentPanel = containerApi.getPanel(projectSummary.id);
    currentPanel?.api.setTitle(projectSummary.name);
  }

  const handleUpdateProjectName = async () => {
    if (!projectSummary) return;

    if (!name || name === projectSummary.name) {
      setName(projectSummary.name);
      return;
    }
    try {
      await projectService.update({
        id: projectSummary.id,
        name,
      });
    } catch (e) {
      console.error("handleUpdateProjectName", e);
      setName(projectSummary.name);
    }
  };
  const handleNameBlur = () => {
    handleUpdateProjectName();
  };

  const handleUpdateProjectRepository = async () => {
    if (!projectSummary) return;

    if (!repository) {
      setRepository("");
      return;
    }

    try {
      await projectService.update({
        id: projectSummary.id,
        // repository: !repository ? "REMOVE" : { UPDATE: repository },
      });
    } catch (e) {
      console.error("handleUpdateProjectRepository", e);
      setRepository("");
    }
  };

  const handleRepositoryBlur = () => {
    handleUpdateProjectRepository();
  };

  if (!projectSummary) {
    return (
      <div className="text-(--moss-primary-foreground) flex h-full items-center justify-center">
        <div className="text-center">
          <h2 className="text-lg font-semibold">No Active Project</h2>
          <p className="text-sm">Please select a project to view its settings.</p>
        </div>
      </div>
    );
  }

  return (
    <div className="relative flex h-full justify-center">
      <div className="w-full max-w-2xl space-y-9 px-6 py-5">
        <div className="space-y-6">
          <div className="text-(--moss-primary-foreground) flex items-start gap-3.5">
            <label className="mt-1 w-20 font-medium">Name:</label>
            <div>
              <Input
                intent="outlined"
                value={name}
                onChange={(e) => setName(e.target.value)}
                onBlur={handleNameBlur}
                onKeyDown={(e) => {
                  if (e.key === "Enter") {
                    e.preventDefault();
                    handleUpdateProjectName();
                    e.currentTarget.blur();
                  }
                }}
                placeholder="Enter project name..."
                pattern={VALID_NAME_PATTERN}
                className="border-(--moss-border) w-72"
              />
              <p className="text-(--moss-secondary-foreground) mt-1 w-72 text-sm">
                Invalid filename characters (e.g. / \ : * ? " &lt; &gt; |) will be escaped
              </p>
            </div>
          </div>

          <div className="text-(--moss-primary-foreground) mt-10 flex items-start gap-3.5">
            <label className="mt-1 w-20 font-medium">Repository:</label>
            <div>
              <Input
                intent="outlined"
                value={repository}
                onChange={(e) => setRepository(e.target.value)}
                onBlur={handleRepositoryBlur}
                onKeyDown={(e) => {
                  if (e.key === "Enter") {
                    e.preventDefault();
                    handleUpdateProjectRepository();
                    e.currentTarget.blur();
                  }
                }}
                placeholder="Enter repository URL..."
                className="border-(--moss-border) w-72"
                required
              />
            </div>
          </div>
        </div>

        <ProjectDangerZoneSection onDeleteClick={openModal} />
      </div>

      {/* Right Column - Summary positioned absolutely on the right */}
      <div className="absolute right-2 top-0 w-60 py-2">
        <ProjectSummarySection />
      </div>

      {showModal && <DeleteProjectModal showModal={showModal} closeModal={closeModal} id={params.projectId} />}
    </div>
  );
};
