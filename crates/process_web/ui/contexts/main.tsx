import {
  Dispatch,
  ReactNode,
  createContext,
  useContext,
  useReducer, useEffect
} from "react";
import {
  CollectConfigAction,
  CollectConfigReducer,
  CollectConfigState,
  initCollectConfigState,
} from "@/contexts/collectCofnigContext";
import {
  CollectLogAction,
  CollectLogReducer,
  CollectLogState,
  initCollectLogState,
} from "@/contexts/collectLogContext";
import {
  initSyncConfigState,
  SyncConfigAction,
  SyncConfigReducer,
  SyncConfigState,
} from "@/contexts/syncCofnigContext";
import {
  initSyncLogState,
  SyncLogAction,
  SyncLogReducer,
  SyncLogState,
} from "@/contexts/syncLogContext";
import {
  DataSourceListAction,
  DataSourceListReducer,
  DataSourceListState,
  initDataSourceListState,
} from "@/contexts/dataSourceListContext";
import {
  DataSharingConfigAction,
  DataSharingConfigReducer,
  DataSharingConfigState,
  initDataSharingConfigState,
} from "@/contexts/dataSharingConfigContext";
import {
  initSharingRequestLogState,
  SharingRequestLogAction,
  SharingRequestLogReducer,
  SharingRequestLogState,
} from "@/contexts/sharingRequestLog";

interface MainState {
  token: string;
  roles: string[];
  permissions: string[];
  userInfo: Record<string, any> | null;
  config: Record<string, any> | null;
  collectConfig: CollectConfigState;
  collectLog: CollectLogState;
  syncConfig: SyncConfigState;
  syncLog: SyncLogState;
  dataSourceList: DataSourceListState;
  dataSharingConfig: DataSharingConfigState;
  sharingRequestLog: SharingRequestLogState;
}

type MainAction =
  | { type: "setConfig"; config: MainState["config"] }
  | { type: "setToken"; token: MainState["token"] }
  | { type: "setRoles"; roles: MainState["roles"] }
  | { type: "setPermissions"; permissions: MainState["permissions"] }
  | { type: "setUserInfo"; userInfo: MainState["userInfo"] }
  | CollectConfigAction
  | CollectLogAction
  | SyncConfigAction
  | SyncLogAction
  | DataSourceListAction
  | DataSharingConfigAction
  | SharingRequestLogAction;

function reducer(state: MainState, action: MainAction) {
  if (action.type === "setConfig") {
    return {
      ...state,
      config: action.config,
    };
  }
  if (action.type === "setToken") {
    return {
      ...state,
      token: action.token,
    };
  }
  if (action.type === "setRoles") {
    return {
      ...state,
      roles: action.roles,
    };
  }
  if (action.type === "setPermissions") {
    return {
      ...state,
      permissions: action.permissions,
    };
  }
  if (action.type === "setUserInfo") {
    return {
      ...state,
      userInfo: action.userInfo,
    };
  }
  if (action.type.indexOf("collectConfig") > -1) {
    return {
      ...state,
      collectConfig: CollectConfigReducer(
        state.collectConfig,
        action as CollectConfigAction
      ),
    };
  }
  if (action.type.indexOf("collectLog") > -1) {
    return {
      ...state,
      collectLog: CollectLogReducer(
        state.collectLog,
        action as CollectLogAction
      ),
    };
  }
  if (action.type.indexOf("syncConfig") > -1) {
    return {
      ...state,
      syncConfig: SyncConfigReducer(
        state.syncConfig,
        action as SyncConfigAction
      ),
    };
  }
  if (action.type.indexOf("syncLog") > -1) {
    return {
      ...state,
      syncLog: SyncLogReducer(state.syncLog, action as SyncLogAction),
    };
  }
  if (action.type.indexOf("dataSourceList") > -1) {
    return {
      ...state,
      dataSourceList: DataSourceListReducer(
        state.dataSourceList,
        action as DataSourceListAction
      ),
    };
  }
  if (action.type.indexOf("dataSharingConfig") > -1) {
    return {
      ...state,
      dataSharingConfig: DataSharingConfigReducer(
        state.dataSharingConfig,
        action as DataSharingConfigAction
      ),
    };
  }

  if (action.type.indexOf("sharingRequestLog") > -1) {
    return {
      ...state,
      sharingRequestLog: SharingRequestLogReducer(
        state.sharingRequestLog,
        action as SharingRequestLogAction
      ),
    };
  }

  throw Error("Unknown action");
}

interface IMainContext {
}

export const MainContext = createContext<{
  state: MainState;
  dispatch: Dispatch<MainAction>;
} | null>(null);

export function MainContextProvider(props: { children: ReactNode }) {
  const [state, dispatch] = useReducer(reducer, {
    token: "",
    roles: [],
    permissions: [],
    userInfo: null,
    collectConfig: initCollectConfigState,
    collectLog: initCollectLogState,
    syncConfig: initSyncConfigState,
    syncLog: initSyncLogState,
    dataSourceList: initDataSourceListState,
    dataSharingConfig: initDataSharingConfigState,
    sharingRequestLog: initSharingRequestLogState,
    config: null
  });

  useEffect(() => {
    fetch(`${window.location.origin}/config.json`)
      .then(res => res.json())
      .then(config => {
        dispatch({
          type: "setConfig",
          config
        })
      });

  }, [])

  return (
    <MainContext.Provider value={{ state, dispatch }}>
      {props.children}
    </MainContext.Provider>
  );
}

export function useMainContext() {
  return useContext(MainContext);
}
