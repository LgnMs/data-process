import {
  Dispatch,
  ReactNode,
  createContext,
  useContext,
  useReducer,
  useEffect,
  useState,
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
import { http_post, ResTemplate } from "@/api/common";
import { reject } from "lodash";

interface AuthInfo {
  name: string;
  auth_id: string;
  auth_secret: string;
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
  }
  if (action.type === "setAuthInfo") {
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
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    fetch(`${window.location.origin}/config.json`)
      .then((res) => res.json())
      .then((config) => {
        dispatch({
          type: "setConfig",
          config,
        });
        document.title = config.title;
      });

    new Promise<string>((resolve, reject) => {
      let authInfoStr;
      let ticks = 0;
      let haveRemoteAuthScript = false;
      let timer = setInterval(() => {
        if (document.getElementById("remote-auth")) {
          haveRemoteAuthScript = true;
          authInfoStr = sessionStorage.getItem("authInfo");
          if (authInfoStr !== null) {
            clearInterval(timer);
            resolve(authInfoStr);
          }
        }

        if (haveRemoteAuthScript && ticks > 15) {
          reject(
            Error(
              "remote-auth.js 中未正确设置authInfo {\n" +
                "  name: string,\n" +
                "  authId: string,\n" +
                "  authSecret: string,\n" +
                "}"
            )
          );
        } else if (ticks > 3) {
          clearInterval(timer);
          authInfoStr =
            '{"name": "admin", "auth_id": "admin", "auth_secret": "admin"}';
          resolve(authInfoStr);
        }
        ticks += 1;
      }, 200);
    }).then((data) => {
      let authInfo: AuthInfo | null = null;
      if (data) {
        try {
          authInfo = JSON.parse(data);
        } catch (_) {
          return reject("remote-auth.js 中设置的authInfo无法转换为JSON");
        }
      }
      dispatch({
        type: "setAuthInfo",
        authInfo,
      });

      http_post<ResTemplate<{ token_type: string; access_token: string }>>(
        "/api/auth/authorize",
        {
          body: JSON.stringify({
            auth_id: authInfo?.auth_id,
            auth_secret: authInfo?.auth_secret,
          }),
        }
      ).then((res) => {
        if (res.data) {
          sessionStorage.setItem(
            "Authorization",
            `${res.data.token_type} ${res.data.access_token}`
          );
        }

        setLoading(false);
      }).catch(err => {
        setLoading(false);
      })

    });
  }, []);

  return (
    !loading && (
      <MainContext.Provider value={{ state, dispatch }}>
        {props.children}
      </MainContext.Provider>
    )
  );
}

export function useMainContext() {
  return useContext(MainContext);
}
