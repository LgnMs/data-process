import { PaginationPayload } from "@/api/common";
import { CollectConfig as ICollectConfig } from "@/api/models/CollectConfig";

export interface CollectConfigState {
    pagination: PaginationPayload<ICollectConfig>
    editFormOpen: boolean,
    editFormData: ICollectConfig | null
}

export type CollectConfigAction =
  | { type: "collectConfig.setPagination"; pagination: CollectConfigState["pagination"] }
  | { type: "collectConfig.setEditFormOpen"; editFormOpen: CollectConfigState["editFormOpen"] }
  | { type: "collectConfig.setEditFormData"; editFormData: CollectConfigState["editFormData"] }

export function CollectConfigReducer(state: CollectConfigState, action: CollectConfigAction) {
    if (action.type === "collectConfig.setPagination") {
        return {
            ...state,
            pagination: action.pagination,
        };
    }
    if (action.type === "collectConfig.setEditFormOpen") {
        return {
            ...state,
            editFormOpen: action.editFormOpen,
        };
    }
    if (action.type === "collectConfig.setEditFormData") {
        return {
            ...state,
            editFormData: action.editFormData,
        };
    }
    throw Error("Unknown action");
}

export const initCollectConfigState: CollectConfigState = {
    pagination: {
        current: 1,
        page_size: 10,
        data: null,
    },
    editFormOpen: false,
    editFormData: null
}
