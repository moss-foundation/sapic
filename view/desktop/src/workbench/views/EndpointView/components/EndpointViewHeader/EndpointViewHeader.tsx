import { useCallback, useContext, useState } from "react";

import { useDescribeProjectResource, useUpdateProjectResource } from "@/adapters";
import { resourceDetailsCollection } from "@/db/resourceDetailsCollection";
import { Button, Icon, MossDropdown, ToggleButton } from "@/lib/ui";
import Select from "@/lib/ui/Select";
import { cn } from "@/utils";
import { useRenameResourceDetailsForm } from "@/workbench/hooks/useRenameResourceDetailsForm";
import { PageWrapper } from "@/workbench/ui/components/PageView/PageWrapper";
import {
  AddPathParamParams,
  AddQueryParamParams,
  UpdatePathParamParams,
  UpdateQueryParamParams,
} from "@repo/moss-project";
import { eq, useLiveQuery } from "@tanstack/react-db";

import { EndpointViewContext } from "../../EndpointViewContext";
import { EditableHeader } from "./EditableHeader";
import { buildDescriptionParamsToAdd, buildPathParamUpdateObject, buildQueryParamUpdateObject } from "./utils";

const optionsPlaceholder = [
  { label: "All", value: "All" },
  { label: "Released", value: "Released" },
  { label: "Draft", value: "Draft" },
  { label: "Archived", value: "Archived" },
  { label: "Some very long name", value: "Some very long name" },
];

export const EndpointViewHeader = () => {
  const { projectId, resourceId } = useContext(EndpointViewContext);

  const [isEnabled, setIsEnabled] = useState(false);
  const [selectedValue, setSelectedValue] = useState("Released");

  const { data: backendResourceDetails } = useDescribeProjectResource({ projectId, resourceId });
  const { mutate: updateProjectResource } = useUpdateProjectResource();

  const { data: localResourceDetails } = useLiveQuery((q) =>
    q
      .from({ collection: resourceDetailsCollection })
      .where(({ collection }) => eq(collection.id, resourceId))
      .findOne()
  );

  const {
    isRenamingResourceDetails,
    setIsRenamingResourceDetails,
    handleRenamingResourceDetailsSubmit,
    handleRenamingResourceDetailsCancel,
  } = useRenameResourceDetailsForm(localResourceDetails);

  const handleSave = useCallback(() => {
    if (!localResourceDetails || !backendResourceDetails) {
      console.warn("Missing required data for save operation");
      return;
    }

    const localPathParams = localResourceDetails.pathParams ?? [];
    const backendPathParams = backendResourceDetails.pathParams ?? [];

    // Build maps for easier lookup
    const localPathParamsById = new Map(localPathParams.map((param) => [param.id, param]));
    const localPathParamsByName = new Map(localPathParams.map((param) => [param.name, param]));
    const backendPathParamsById = new Map(backendPathParams.map((param) => [param.id, param]));
    const backendPathParamsByName = new Map(backendPathParams.map((param) => [param.name, param]));

    // Find path params to add (exist in local but not in backend by id or name)
    const pathParamsToAdd: AddPathParamParams[] = localPathParams
      .filter((localParam) => {
        const existsById = backendPathParamsById.has(localParam.id);
        const existsByName = backendPathParamsByName.has(localParam.name);
        return !existsById && !existsByName;
      })
      .map((param) => ({
        name: param.name,
        value: param.value,
        order: param.order ?? 0,
        description: param.description,
        options: {
          disabled: param.disabled ?? false,
          propagate: param.propagate ?? false,
        },
      }));

    // Find path params to update (exist in both but have different values)
    const pathParamsToUpdate: UpdatePathParamParams[] = localPathParams
      .map((localParam) => {
        const backendParam = backendPathParamsById.get(localParam.id) ?? backendPathParamsByName.get(localParam.name);
        if (!backendParam) return null;

        const updateObj = buildPathParamUpdateObject(backendParam, localParam);
        // Only include if there are actual changes (more than just the id)
        return Object.keys(updateObj).length > 1 ? updateObj : null;
      })
      .filter((updateObj): updateObj is UpdatePathParamParams => updateObj !== null);

    // Find path params to remove (exist in backend but not in local)
    const pathParamsToRemove: string[] = backendPathParams
      .filter((backendParam) => {
        const existsById = localPathParamsById.has(backendParam.id);
        const existsByName = localPathParamsByName.has(backendParam.name);
        return !existsById && !existsByName;
      })
      .map((param) => param.id);

    const localQueryParams = localResourceDetails.queryParams ?? [];
    const backendQueryParams = backendResourceDetails.queryParams ?? [];

    // Build maps for easier lookup
    const localQueryParamsById = new Map(localQueryParams.map((param) => [param.id, param]));
    const localQueryParamsByName = new Map(localQueryParams.map((param) => [param.name, param]));
    const backendQueryParamsById = new Map(backendQueryParams.map((param) => [param.id, param]));
    const backendQueryParamsByName = new Map(backendQueryParams.map((param) => [param.name, param]));

    // Find query params to add (exist in local but not in backend by id or name)
    const queryParamsToAdd: AddQueryParamParams[] = localQueryParams
      .filter((localParam) => {
        const existsById = backendQueryParamsById.has(localParam.id);
        const existsByName = backendQueryParamsByName.has(localParam.name);
        return !existsById && !existsByName;
      })
      .map((param) => ({
        name: param.name,
        value: param.value,
        order: param.order ?? 0,
        description: param.description,
        options: {
          disabled: param.disabled ?? false,
          propagate: param.propagate ?? false,
        },
      }));

    // Find query params to update (exist in both but have different values)
    const queryParamsToUpdate: UpdateQueryParamParams[] = localQueryParams
      .map((localParam) => {
        const backendParam = backendQueryParamsById.get(localParam.id) ?? backendQueryParamsByName.get(localParam.name);
        if (!backendParam) return null;

        const updateObj = buildQueryParamUpdateObject(backendParam, localParam);
        // Only include if there are actual changes (more than just the id)
        return Object.keys(updateObj).length > 1 ? updateObj : null;
      })
      .filter((updateObj): updateObj is UpdateQueryParamParams => updateObj !== null);

    // Find query params to remove (exist in backend but not in local)
    const queryParamsToRemove: string[] = backendQueryParams
      .filter((backendParam) => {
        const existsById = localQueryParamsById.has(backendParam.id);
        const existsByName = localQueryParamsByName.has(backendParam.name);
        return !existsById && !existsByName;
      })
      .map((param) => param.id);

    const descriptionParamsToAdd = buildDescriptionParamsToAdd(localResourceDetails, backendResourceDetails);

    // Only proceed if there are changes to save
    if (
      pathParamsToAdd.length === 0 &&
      pathParamsToUpdate.length === 0 &&
      pathParamsToRemove.length === 0 &&
      queryParamsToAdd.length === 0 &&
      queryParamsToUpdate.length === 0 &&
      queryParamsToRemove.length === 0 &&
      descriptionParamsToAdd === null
    ) {
      console.log("No resource description changes to save");
      return;
    }

    try {
      if (localResourceDetails.kind === "Item") {
        updateProjectResource({
          projectId,
          updatedResource: {
            ITEM: {
              id: resourceId,
              ...descriptionParamsToAdd,
              headersToAdd: [],
              headersToUpdate: [],
              headersToRemove: [],
              pathParamsToAdd,
              pathParamsToUpdate,
              pathParamsToRemove,
              queryParamsToAdd,
              queryParamsToUpdate,
              queryParamsToRemove,
            },
          },
        });
      } else {
        console.warn(`Only "Item" kind of resources can be updated currently`);
      }
    } catch (error) {
      console.error("Error updating path params and query params:", error);
    }
  }, [localResourceDetails, backendResourceDetails, updateProjectResource, projectId, resourceId]);

  if (!localResourceDetails) return null;

  return (
    <PageWrapper>
      <header className="flex flex-col gap-3">
        <div className="flex items-center justify-between">
          <EditableHeader
            icon="Http"
            title={localResourceDetails.name}
            isRenamingResourceDetails={isRenamingResourceDetails}
            setIsRenamingResourceDetails={setIsRenamingResourceDetails}
            handleRenamingResourceDetailsSubmit={handleRenamingResourceDetailsSubmit}
            handleRenamingResourceDetailsCancel={handleRenamingResourceDetailsCancel}
            editable
          />
          <div className="flex items-center gap-2">
            <Button intent="outlined" onClick={handleSave}>
              Save
            </Button>

            <ToggleButton checked={isEnabled} onCheckedChange={setIsEnabled} />
            <Select.Root value={selectedValue} onValueChange={setSelectedValue}>
              <Select.Trigger
                placeholder="Select an option"
                childrenLeftSide={
                  <span
                    className={cn("size-1.5 rounded-full", {
                      "background-(--moss-blue-4)": selectedValue === "Released",
                      "background-(--moss-orange-5)": selectedValue === "Draft",
                      "background-(--moss-error)": selectedValue === "Archived",
                      "hidden": selectedValue === "Some very long name",
                    })}
                  />
                }
              />

              <Select.Content align="end">
                {optionsPlaceholder?.map((option) => (
                  <Select.Item key={option.value} value={option.value}>
                    {option.label}
                  </Select.Item>
                ))}
              </Select.Content>
            </Select.Root>

            <MossDropdown.Root>
              <MossDropdown.Trigger>
                <Icon icon="MoreHorizontal" />
              </MossDropdown.Trigger>
              <MossDropdown.Portal>
                <MossDropdown.Content>
                  <MossDropdown.Item>Item 1</MossDropdown.Item>
                  <MossDropdown.Item>Item 2</MossDropdown.Item>
                  <MossDropdown.Item>Item 3</MossDropdown.Item>
                </MossDropdown.Content>
              </MossDropdown.Portal>
            </MossDropdown.Root>
          </div>
        </div>

        <div className="flex items-center gap-5">
          <div className="flex gap-[3px]">
            <span className="text-(--moss-primary-descriptionForeground)">Created</span> <span>March 31, 2025</span>
          </div>
          <div className="flex gap-[3px]">
            <span className="text-(--moss-primary-descriptionForeground)">Updated</span> <span>March 31, 2025</span>
          </div>
        </div>
      </header>
    </PageWrapper>
  );
};
