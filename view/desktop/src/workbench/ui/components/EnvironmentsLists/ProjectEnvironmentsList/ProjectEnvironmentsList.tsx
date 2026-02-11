import { useListProjects } from "@/adapters/tanstackQuery/project/useListProjects";

import { ProjectEnvironmentsListRoot } from "./ProjectEnvironmentsListRoot";

export const ProjectEnvironmentsList = () => {
  const { data: projects } = useListProjects();

  return (
    <div>
      {projects?.items.map((project) => (
        <ProjectEnvironmentsListRoot key={project.id} projectId={project.id} />
      ))}
    </div>
  );
};
