import { useEffect, useRef, useState } from "react";

import { useCreateProject, useCreateProjectResource, useDeleteProjectResource, useStreamProjects } from "@/adapters";
import { cn } from "@/utils";
import { IconInline } from "@/workbench/ui/components/IconInline";
import {
  convertResourceInfoToCreateInput,
  getAllNestedResources,
  getSourceProjectTreeNodeData,
  isSourceProjectTreeNode,
} from "@/workbench/ui/components/ProjectTree/utils";
import { dropTargetForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";
import { join } from "@tauri-apps/api/path";

export const ProjectCreationZone = () => {
  const ref = useRef<HTMLDivElement>(null);

  const [canDrop, setCanDrop] = useState<boolean | null>(null);

  const { mutateAsync: createProject } = useCreateProject();
  const { mutateAsync: createProjectResource } = useCreateProjectResource();
  const { mutateAsync: deleteProjectResource } = useDeleteProjectResource();
  const { data: projects } = useStreamProjects();

  useEffect(() => {
    const element = ref.current;
    if (!element) return;

    return dropTargetForElements({
      element,
      getData: () => ({
        type: "ProjectCreationZone",
        data: {},
      }),
      canDrop({ source }) {
        return isSourceProjectTreeNode(source);
      },
      onDragEnter() {
        setCanDrop(true);
      },
      onDragLeave() {
        setCanDrop(null);
      },
      onDrop: async ({ source }) => {
        setCanDrop(null);

        const sourceTarget = getSourceProjectTreeNodeData(source);

        if (!sourceTarget) return;

        const resources = getAllNestedResources(sourceTarget.node);

        if (resources.length === 0) return;

        const rootResource = resources[0];
        const nestedResources = resources.slice(1);

        const newProject = await createProject({
          name: rootResource.name,
          order: (projects?.length ?? 0) + 1,
        });

        try {
          await deleteProjectResource({
            projectId: sourceTarget.projectId,
            input: { id: rootResource.id },
          });
        } catch (error) {
          console.error("Error during project creation:", error);
        }

        try {
          for (const [index, resource] of nestedResources.entries()) {
            const rootResourceName = rootResource.name;
            let adjustedSegments = resource.path.segments;

            const rootNameIndex = adjustedSegments.findIndex((segment) => segment === rootResourceName);
            if (rootNameIndex !== -1) {
              adjustedSegments = [
                ...adjustedSegments.slice(0, rootNameIndex),
                ...adjustedSegments.slice(rootNameIndex + 1),
              ];
            }

            const parentSegments = adjustedSegments.slice(0, -1);
            const parentPath = parentSegments.length > 0 ? await join(...parentSegments) : "";

            const createInput = convertResourceInfoToCreateInput(resource, parentPath);

            createInput[resource.kind === "Dir" ? "DIR" : "ITEM"].order = index + 1;

            await createProjectResource({
              projectId: newProject.id,
              input: createInput,
            });
          }
        } catch (error) {
          console.error("Error during project creation:", error);
        }
      },
    });
  }, [projects?.length, createProject, createProjectResource, deleteProjectResource]);

  return (
    <div
      ref={ref}
      className={cn(
        "background-(--moss-accent-secondary) border-(--moss-accent) grid h-max min-h-32 w-full place-items-center rounded border-2 border-dashed transition-[translate] duration-100",
        {
          "background-(--moss-accent-secondary) -translate-y-1": canDrop === true,
        }
      )}
    >
      <div className="bg-size-[20px_20px] animate-stripes flex flex-col items-center justify-center gap-3 bg-[linear-gradient(-45deg,white_5%,transparent_5%_45%,white_45%_55%,transparent_55%_95%,white_95%)] p-8 text-center">
        <IconInline icon="AddCircleActive" className={cn("text-(--moss-accent) size-5 rounded-full")} />
        <span>Drag & drop selected items here to create a new project</span>
      </div>
    </div>
  );
};
