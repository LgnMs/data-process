import { PaginationPayload } from "@/api/common";
import { CollectConfig as ICollectConfig } from "@/api/models/CollectConfig";
import { createContext, Dispatch } from "react";

export interface CollectConfigState {
    pagination: PaginationPayload<ICollectConfig>
}

export type CollectConfigAction =
  | { type: "collectConfig.setPagination"; pagination: CollectConfigState["pagination"] }

export function CollectConfigReducer(state: CollectConfigState, action: CollectConfigAction) {
    if (action.type === "collectConfig.setPagination") {
        return {
            ...state,
            pagination: action.pagination,
        };
    }
    throw Error("Unknown action");
}

export const initCollectConfigState: CollectConfigState = {
    pagination: {
        current: 1,
        page_size: 10,
        data: null,
    }
}
