import {
  Dispatch,
  ReactNode,
  createContext,
  useContext,
  useEffect,
  useReducer,
} from "react";

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
}

type MainAction =
  | { type: "setToken"; token: MainState["token"] }
  | { type: "setRoles"; roles: MainState["roles"] }
  | { type: "setPermissions"; permissions: MainState["permissions"] }
  | { type: "setUserInfo"; userInfo: MainState["userInfo"] };

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
