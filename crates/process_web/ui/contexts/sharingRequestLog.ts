import { PaginationPayload } from "@/api/common";
import { SharingRequestLog as ISharingRequestLog } from "@/api/models/SharingRequestLog";
import { SharingRequestLogParams } from "@/api/models/SharingRequestLogParams";

export interface SharingRequestLogState {
  pagination: PaginationPayload<SharingRequestLogParams>;
  drawerOpen: boolean;
  drawerData: ISharingRequestLog | null;
}

export type SharingRequestLogAction =
  | {
      type: "sharingRequestLog.setPagination";
      pagination: SharingRequestLogState["pagination"];
    }
  | {
      type: "sharingRequestLog.setDrawerOpen";
      drawerOpen: SharingRequestLogState["drawerOpen"];
    }
  | {
      type: "sharingRequestLog.setDrawerData";
      drawerData: SharingRequestLogState["drawerData"];
    };

export function SharingRequestLogReducer(
  state: SharingRequestLogState,
  action: SharingRequestLogAction
) {
  if (action.type === "sharingRequestLog.setPagination") {
    return {
      ...state,
      pagination: action.pagination,
    };
  }
  if (action.type === "sharingRequestLog.setDrawerOpen") {
    return {
      ...state,
      drawerOpen: action.drawerOpen,
    };
  }
  if (action.type === "sharingRequestLog.setDrawerData") {
    return {
      ...state,
      drawerData: action.drawerData,
    };
  }
  throw Error("Unknown action");
}

export const initSharingRequestLogState: SharingRequestLogState = {
  pagination: {
    current: 1,
    page_size: 10,
    data: null,
  },
  drawerOpen: false,
  drawerData: null,
};
