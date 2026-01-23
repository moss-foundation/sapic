import { z } from "zod";

import { projectService } from "@/domains/project/projectService";
import { treeItemStateService } from "@/workbench/domains/treeItemState/service";
import { StreamProjectsEvent } from "@repo/ipc";
import { createCollection } from "@tanstack/db";
import { QueryClient } from "@tanstack/query-core";
import { queryCollectionOptions } from "@tanstack/query-db-collection";
import { Channel } from "@tauri-apps/api/core";

import { projectSummarySchema } from "./schemas/projectSummarySchema";

declare module "@tanstack/query-db-collection" {
  interface QueryCollectionMeta {
    workspaceId?: string;
  }
}

const queryClient = new QueryClient();

type ProjectSummary = z.infer<typeof projectSummarySchema>;

type ProjectSummaryChanges = Partial<Pick<ProjectSummary, "expanded" | "order">>;

export const projectSummariesCollection = createCollection(
  queryCollectionOptions({
    queryClient,

    id: "projectSummaries",
    queryKey: ["projectSummaries"],

    getKey: (item) => item.id,
    schema: projectSummarySchema,
    queryFn: async (): Promise<ProjectSummary[]> => {
      const workspaceFromHash = window.location.hash.split("/").pop();

      if (!workspaceFromHash) return [];

      const projects: ProjectSummary[] = [];

      const projectEvent = new Channel<StreamProjectsEvent>();
      projectEvent.onmessage = (project) => {
        const projectSummary = {
          ...project,
          branch: project.branch ?? undefined,
          order: project.order ?? 228,
          expanded: project.expanded ?? false,
          archived: project.archived ?? false,
          iconPath: project.iconPath ?? undefined,
        } satisfies ProjectSummary;

        projects.push(projectSummary);
      };

      await projectService.streamProjects(projectEvent);

      const treeItemStates = await treeItemStateService.batchGet(
        projects.map((p) => p.id),
        workspaceFromHash ?? "application"
      );

      const projectsWithTreeItemStates = projects.map((project) => {
        const treeItemState = treeItemStates.find((t) => t.id === project.id);
        return {
          ...project,
          expanded: treeItemState?.expanded ?? false,
          order: treeItemState?.order ?? -1,
        };
      });

      return projectsWithTreeItemStates;
    },

    onUpdate: async ({ transaction }) => {
      console.log("onUpdate", transaction);
      const updates = transaction.mutations.map(
        (
          m
        ): {
          id: string;
          changes: ProjectSummaryChanges;
          original: ProjectSummary;
          metadata: { workspaceId?: string };
        } => ({
          id: m.key,
          changes: m.changes as ProjectSummaryChanges,
          original: m.original as ProjectSummary,
          metadata: m.metadata as { workspaceId?: string },
        })
      );
      const workspaceId = updates[0].metadata.workspaceId ?? "application";

      // console.log("updates", {
      //   updates: updates.map((u) => ({
      //     id: u.id,
      //     expanded: u.changes.expanded ?? u.original.expanded,
      //     order: u.changes.order ?? u.original.order,
      //   })),
      //   workspaceId,
      // });

      if (updates.length === 1) {
        const update = updates[0];
        await treeItemStateService.put(
          {
            id: update.id,
            expanded: update.changes.expanded ?? update.original.expanded,
            order: update.changes.order ?? update.original.order,
          },
          workspaceId
        );

        projectSummariesCollection.utils.writeUpdate({
          id: update.id,
          ...update.changes,
        });
      } else if (updates.length > 1) {
        await treeItemStateService.batchPut(
          updates.map((u) => ({
            id: u.id,
            expanded: u.changes.expanded ?? u.original.expanded,
            order: u.changes.order ?? u.original.order,
          })),
          workspaceId
        );
        projectSummariesCollection.utils.writeUpdate(
          updates.map((u) => ({
            id: u.id,
            ...u.changes,
          }))
        );
      }

      return { refetch: false };
    },
  })
);
