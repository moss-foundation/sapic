// Create a new router instance
import MainIndex from "@/main";
import WorkspaceShell from "@/pages/main/WorkspaceShell";
import NotFoundPage from "@/welcome/pages/NotFoundPage";
import { createHashHistory, createRootRoute, createRoute, createRouter } from "@tanstack/react-router";

const rootRoute = createRootRoute({
  component: MainIndex,
  notFoundComponent: NotFoundPage,
});

const workspaceIdRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: "/$workspaceId",
  component: WorkspaceShell,
});

export const mainRouter = createRouter({
  routeTree: rootRoute.addChildren([workspaceIdRoute]),
  history: createHashHistory(),
});

// Register the router instance for type safety
declare module "@tanstack/react-router" {
  interface Register {
    router: typeof mainRouter;
  }
}
