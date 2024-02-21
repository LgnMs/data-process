import { PaginationPayload } from "@/api/common";
import { SyncConfig as ISyncConfig } from "@/api/models/SyncConfig";
import { SyncConfigListParams } from "@/api/models/SyncConfigListParams";

export interface SyncConfigState {
  pagination: PaginationPayload<SyncConfigListParams>;
  editFormOpen: boolean;
  editFormData: ISyncConfig | null;
}

export type SyncConfigAction =
  | {
      type: "syncConfig.setPagination";
      pagination: SyncConfigState["pagination"];
    }
  | {
      type: "syncConfig.setEditFormOpen";
      editFormOpen: SyncConfigState["editFormOpen"];
    }
  | {
      type: "syncConfig.setEditFormData";
      editFormData: SyncConfigState["editFormData"];
    };

export function SyncConfigReducer(
  state: SyncConfigState,
  action: SyncConfigAction
) {
  if (action.type === "syncConfig.setPagination") {
    return {
      ...state,
      pagination: action.pagination,
    };
  }
  if (action.type === "syncConfig.setEditFormOpen") {
    return {
      ...state,
      editFormOpen: action.editFormOpen,
    };
  }
  if (action.type === "syncConfig.setEditFormData") {
    return {
      ...state,
      editFormData: action.editFormData,
    };
  }
  throw Error("Unknown action");
}

export const initSyncConfigState: SyncConfigState = {
  pagination: {
    current: 1,
    page_size: 10,
    data: null,
  },
  editFormOpen: false,
  editFormData: null,
};
