import { PaginationPayload } from "@/api/common";
import { CollectLog as ICollectLog } from "@/api/models/CollectLog";

export interface CollectLogState {
  pagination: PaginationPayload<ICollectLog>;
  drawerOpen: boolean;
  drawerData: ICollectLog | null;
}

export type CollectLogAction =
  | {
      type: "collectLog.setPagination";
      pagination: CollectLogState["pagination"];
    }
  | {
      type: "collectLog.setDrawerOpen";
      drawerOpen: CollectLogState["drawerOpen"];
    }
  | {
      type: "collectLog.setDrawerData";
      drawerData: CollectLogState["drawerData"];
    };

export function CollectLogReducer(
  state: CollectLogState,
  action: CollectLogAction
) {
  if (action.type === "collectLog.setPagination") {
    return {
      ...state,
      pagination: action.pagination,
    };
  }
  if (action.type === "collectLog.setDrawerOpen") {
    return {
      ...state,
      drawerOpen: action.drawerOpen,
    };
  }
  if (action.type === "collectLog.setDrawerData") {
    return {
      ...state,
      drawerData: action.drawerData,
    };
  }
  throw Error("Unknown action");
}

export const initCollectLogState: CollectLogState = {
  pagination: {
    current: 1,
    page_size: 10,
    data: null,
  },
  drawerOpen: false,
  drawerData: null,
};
