import { PaginationPayload } from "@/api/common";
import { SyncLog as ISyncLog } from "@/api/models/SyncLog";
import { SyncLogListParams } from "@/api/models/SyncLogListParams";

export interface SyncLogState {
  pagination: PaginationPayload<SyncLogListParams>;
  drawerOpen: boolean;
  drawerData: ISyncLog | null;
}

export type SyncLogAction =
  | {
      type: "syncLog.setPagination";
      pagination: SyncLogState["pagination"];
    }
  | {
      type: "syncLog.setDrawerOpen";
      drawerOpen: SyncLogState["drawerOpen"];
    }
  | {
      type: "syncLog.setDrawerData";
      drawerData: SyncLogState["drawerData"];
    };

export function SyncLogReducer(state: SyncLogState, action: SyncLogAction) {
  if (action.type === "syncLog.setPagination") {
    return {
      ...state,
      pagination: action.pagination,
    };
  }
  if (action.type === "syncLog.setDrawerOpen") {
    return {
      ...state,
      drawerOpen: action.drawerOpen,
    };
  }
  if (action.type === "syncLog.setDrawerData") {
    return {
      ...state,
      drawerData: action.drawerData,
    };
  }
  throw Error("Unknown action");
}

export const initSyncLogState: SyncLogState = {
  pagination: {
    current: 1,
    page_size: 10,
    data: null,
  },
  drawerOpen: false,
  drawerData: null,
};
