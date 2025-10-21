import { useEffect, useRef, useState } from "react";

import {
  convertEntryInfoToCreateInput,
  getAllNestedEntries,
  getSourceProjectTreeNodeData,
  isSourceProjectTreeNode,
} from "@/components/ProjectTree/utils";
import { useCreateProject, useCreateProjectEntry, useDeleteProjectEntry, useStreamProjects } from "@/hooks";
import { Icon } from "@/lib/ui";
import { cn } from "@/utils";
import { dropTargetForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";
import { join } from "@tauri-apps/api/path";

export const ProjectCreationZone = () => {
  const ref = useRef<HTMLDivElement>(null);

  const [canDrop, setCanDrop] = useState<boolean | null>(null);

  const { mutateAsync: createProject } = useCreateProject();
  const { mutateAsync: createProjectEntry } = useCreateProjectEntry();
  const { mutateAsync: deleteProjectEntry } = useDeleteProjectEntry();
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

        const entries = getAllNestedEntries(sourceTarget.node);

        if (entries.length === 0) return;

        const rootEntry = entries[0];
        const nestedEntries = entries.slice(1);

        const newProject = await createProject({
          name: rootEntry.name,
          order: (projects?.length ?? 0) + 1,
        });

        try {
          await deleteProjectEntry({
            projectId: sourceTarget.projectId,
            input: { id: rootEntry.id },
          });
        } catch (error) {
          console.error("Error during project creation:", error);
        }

        try {
          for (const [index, entry] of nestedEntries.entries()) {
            const rootEntryName = rootEntry.name;
            let adjustedSegments = entry.path.segments;

            const rootNameIndex = adjustedSegments.findIndex((segment) => segment === rootEntryName);
            if (rootNameIndex !== -1) {
              adjustedSegments = [
                ...adjustedSegments.slice(0, rootNameIndex),
                ...adjustedSegments.slice(rootNameIndex + 1),
              ];
            }

            const parentSegments = adjustedSegments.slice(0, -1);
            const parentPath = parentSegments.length > 0 ? await join(...parentSegments) : "";

            const createInput = convertEntryInfoToCreateInput(entry, parentPath);

            createInput[entry.kind === "Dir" ? "DIR" : "ITEM"].order = index + 1;

            await createProjectEntry({
              projectId: newProject.id,
              input: createInput,
            });
          }
        } catch (error) {
          console.error("Error during project creation:", error);
        }
      },
    });
  }, [projects?.length, createProject, createProjectEntry, deleteProjectEntry]);

  return (
    <div
      ref={ref}
      className={cn(
        "background-(--moss-accent-secondary) grid h-max min-h-32 w-full place-items-center rounded border-2 border-dashed border-(--moss-accent) transition-[translate] duration-100",
        {
          "background-(--moss-accent-secondary) -translate-y-1": canDrop === true,
        }
      )}
    >
      <div className="animate-stripes flex flex-col items-center justify-center gap-3 bg-[linear-gradient(-45deg,white_5%,transparent_5%_45%,white_45%_55%,transparent_55%_95%,white_95%)] bg-size-[20px_20px] p-8 text-center">
        <Icon icon="AddCircleActive" className={cn("size-5 rounded-full text-(--moss-accent)")} />
        <span>Drag & drop selected items here to create a new project</span>
      </div>
    </div>
  );
};
