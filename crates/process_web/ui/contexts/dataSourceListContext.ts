import { PaginationPayload } from "@/api/common";
import { DataSourceList as IDataSourceList } from "@/api/models/DataSourceList";
import { DataSourceListParams } from "@/api/models/DataSourceListParams";

export interface DataSourceListState {
  pagination: PaginationPayload<DataSourceListParams>;
  editFormOpen: boolean;
  editFormData: IDataSourceList | null;
}

export type DataSourceListAction =
  | {
      type: "dataSourceList.setPagination";
      pagination: DataSourceListState["pagination"];
    }
  | {
      type: "dataSourceList.setEditFormOpen";
      editFormOpen: DataSourceListState["editFormOpen"];
    }
  | {
      type: "dataSourceList.setEditFormData";
      editFormData: DataSourceListState["editFormData"];
    };

export function DataSourceListReducer(
  state: DataSourceListState,
  action: DataSourceListAction
) {
  if (action.type === "dataSourceList.setPagination") {
    return {
      ...state,
      pagination: action.pagination,
    };
  }
  if (action.type === "dataSourceList.setEditFormOpen") {
    return {
      ...state,
      editFormOpen: action.editFormOpen,
    };
  }
  if (action.type === "dataSourceList.setEditFormData") {
    return {
      ...state,
      editFormData: action.editFormData,
    };
  }
  throw Error("Unknown action");
}

export const initDataSourceListState: DataSourceListState = {
  pagination: {
    current: 1,
    page_size: 10,
    data: null,
  },
  editFormOpen: false,
  editFormData: null,
};
