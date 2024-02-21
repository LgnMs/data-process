import {
  Dispatch,
  ReactNode,
  createContext,
  useContext,
  useReducer,
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

export interface IRoleInfo {
  id: string;
  roleCode: string;
  roleDesc: string;
  roleName: string;
}

interface MainState {
  token: string;
  roles: string[];
  permissions: string[];
  userInfo: Record<string, any> | null;
  collectConfig: CollectConfigState;
  collectLog: CollectLogState;
  syncConfig: SyncConfigState;
  syncLog: SyncLogState;
}

type MainAction =
  | { type: "setToken"; token: MainState["token"] }
  | { type: "setRoles"; roles: MainState["roles"] }
  | { type: "setPermissions"; permissions: MainState["permissions"] }
  | { type: "setUserInfo"; userInfo: MainState["userInfo"] }
  | CollectConfigAction
  | CollectLogAction
  | SyncConfigAction
  | SyncLogAction;

function reducer(state: MainState, action: MainAction) {
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

  throw Error("Unknown action");
}

// TODO 处理原来的用户信息
interface IMainContext {
  roleInfos: IRoleInfo[];
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
  });

  return (
    <MainContext.Provider value={{ state, dispatch }}>
      {props.children}
    </MainContext.Provider>
  );
}

export function useMainContext() {
  return useContext(MainContext);
}
