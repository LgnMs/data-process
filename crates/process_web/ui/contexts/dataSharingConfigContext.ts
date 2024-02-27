import { PaginationPayload } from "@/api/common";
import { DataSharingConfig as IDataSharingConfig } from "@/api/models/DataSharingConfig";
import { DataSharingConfigParams } from "@/api/models/DataSharingConfigParams";

export interface DataSharingConfigState {
  pagination: PaginationPayload<DataSharingConfigParams>;
  editFormOpen: boolean;
  editFormData: IDataSharingConfig | null;
}

export type DataSharingConfigAction =
  | {
      type: "dataSharingConfig.setPagination";
      pagination: DataSharingConfigState["pagination"];
    }
  | {
      type: "dataSharingConfig.setEditFormOpen";
      editFormOpen: DataSharingConfigState["editFormOpen"];
    }
  | {
      type: "dataSharingConfig.setEditFormData";
      editFormData: DataSharingConfigState["editFormData"];
    };

export function DataSharingConfigReducer(
  state: DataSharingConfigState,
  action: DataSharingConfigAction
) {
  if (action.type === "dataSharingConfig.setPagination") {
    return {
      ...state,
      pagination: action.pagination,
    };
  }
  if (action.type === "dataSharingConfig.setEditFormOpen") {
    return {
      ...state,
      editFormOpen: action.editFormOpen,
    };
  }
  if (action.type === "dataSharingConfig.setEditFormData") {
    return {
      ...state,
      editFormData: action.editFormData,
    };
  }
  throw Error("Unknown action");
}

export const initDataSharingConfigState: DataSharingConfigState = {
  pagination: {
    current: 1,
    page_size: 10,
    data: null,
  },
  editFormOpen: false,
  editFormData: null,
};
