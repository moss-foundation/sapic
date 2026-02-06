import { useStreamProjects } from "@/adapters";

import { ProjectEnvironmentsListRoot } from "./ProjectEnvironmentsListRoot";

export const ProjectEnvironmentsList = () => {
  const { data: projects } = useStreamProjects();

  return (
    <div>
      {projects?.map((project) => (
        <ProjectEnvironmentsListRoot key={project.id} projectId={project.id} />
      ))}
    </div>
  );
};
