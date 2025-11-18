import { IDockviewPanelProps } from "moss-tabs";
import { useEffect, useState } from "react";

import { VALID_NAME_PATTERN } from "@/constants/validation";
import { useModal } from "@/hooks";
import Input from "@/lib/ui/Input";
import { useStreamProjects, useUpdateProject } from "@/workbench/adapters/tanstackQuery/project";
import { DeleteProjectModal } from "@/workbench/ui/components/Modals/Project/DeleteProjectModal";

import { ProjectDangerZoneSection } from "../ProjectDangerZoneSection";
import { ProjectSummarySection } from "../ProjectSummarySection";

interface OverviewTabContentProps {
  projectId: string;
}

export const OverviewTabContent = ({ params, containerApi }: IDockviewPanelProps<OverviewTabContentProps>) => {
  const { data: streamedProjects } = useStreamProjects();
  const { mutateAsync: updateProject } = useUpdateProject();

  const project = streamedProjects?.find((p) => p.id === params.projectId);

  const { showModal, closeModal, openModal } = useModal();

  const [name, setName] = useState(project?.name || "");
  const [repository, setRepository] = useState("");

  useEffect(() => {
    if (project) {
      setName(project.name);
      setRepository("");
      const currentPanel = containerApi.getPanel(project.id);
      currentPanel?.api.setTitle(project.name);
    }
  }, [project, containerApi]);

  const handleUpdateprojectName = async () => {
    if (!project) return;

    if (!name || name === project.name) {
      setName(project?.name);
      return;
    }
    try {
      await updateProject({
        id: project.id,
        name,
      });
    } catch (e) {
      console.error("handleUpdateProjectName", e);
      setName(project?.name);
    }
  };
  const handleNameBlur = () => {
    handleUpdateprojectName();
  };

  const handleUpdateProjectRepository = async () => {
    if (!project) return;

    if (!repository) {
      setRepository("");
      return;
    }

    try {
      await updateProject({
        id: project.id,
        repository: !repository ? "REMOVE" : { UPDATE: repository },
      });
    } catch (e) {
      console.error("handleUpdateProjectRepository", e);
      setRepository("");
    }
  };

  const handleRepositoryBlur = () => {
    handleUpdateProjectRepository();
  };

  if (!project) {
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
                    handleUpdateprojectName();
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
