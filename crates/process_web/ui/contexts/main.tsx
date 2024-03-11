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


interface AuthInfo {
  name: string,
  authId: string,
}

interface MainState {
  config: Record<string, any> | null;
  // 认证信息，启用USE_REMOTE_AUTH后在public/remote-auth.js 中存入sessionStore
  authInfo: AuthInfo | null;
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
  | { type: "setAuthInfo"; authInfo: MainState["authInfo"] }
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
  }  if (action.type === "setAuthInfo") {
    return {
      ...state,
      authInfo: action.authInfo,
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

export const MainContext = createContext<{
  state: MainState;
  dispatch: Dispatch<MainAction>;
} | null>(null);

export function MainContextProvider(props: { children: ReactNode }) {
  const [state, dispatch] = useReducer(reducer, {
    collectConfig: initCollectConfigState,
    collectLog: initCollectLogState,
    syncConfig: initSyncConfigState,
    syncLog: initSyncLogState,
    dataSourceList: initDataSourceListState,
    dataSharingConfig: initDataSharingConfigState,
    sharingRequestLog: initSharingRequestLogState,
    config: null,
    authInfo: null,
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

    let authInfoStr = sessionStorage.getItem("authInfo");
    let authInfo: AuthInfo | null = null;
    if (authInfoStr) {
      try  {
        authInfo = JSON.parse(authInfoStr);
      } catch (_) {
      }
    }
    dispatch({
      type: 'setAuthInfo',
      authInfo
    })

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
