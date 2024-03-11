// 当USE_REMOTE_AUTH=true时加载这个脚本
(() => {
  const USER_ADMIN_HOST = "http://127.0.0.1:8900";
  // 若后端不支持CORS访问，请求本地/remote-auth将进行代理转发
  const USER_ADMIN_API_HOST = location.origin + "/remote-auth";
  function hasToken() {
    const params = new URLSearchParams(window.location.search);
    const windowUrlParams = new URLSearchParams(params.toString());
    const token = windowUrlParams.get("token");

    return token || window.sessionStorage.getItem("remote_auth_token");
  }

  window.auth = {
    login() {
      if (typeof window === "undefined") {
        return;
      }
      const { location } = window;
      const originUrl = `${location.origin}${
        location.pathname === "/" ? "" : location.pathname
      }`;
      window.location.href = `${USER_ADMIN_HOST}/login?remoteUrl=${originUrl}`;
    },
    logout() {
      const { location } = window;
      const originUrl = `${location.origin}${
        location.pathname === "/" ? "" : location.pathname
      }`;
      window.sessionStorage.removeItem("remote_auth_token");
      window.sessionStorage.removeItem("authInfo");
      window.location.href = `${USER_ADMIN_HOST}/logout?remoteUrl=${originUrl}`;
    },
  };

  if (!hasToken()) {
    window.auth.login();
  } else {
    const params = new URLSearchParams(window.location.search);
    const windowUrlParams = new URLSearchParams(params.toString());
    const token = windowUrlParams.get("token");
    if (token) {
      window.sessionStorage.setItem("remote_auth_token", token);
      windowUrlParams.delete("token");
      window.history.replaceState(
        {},
        "",
        window.location.origin +
          window.location.pathname +
          windowUrlParams.toString()
      );
    }
    fetch(`${USER_ADMIN_API_HOST}/getInfo`, {
      headers: {
        Authorization:
          "Bearer " + window.sessionStorage.getItem("remote_auth_token"),
      },
    })
      .then((res) => {
        return res.json();
      })
      .then((data) => {
        console.log(data);
        if (data.code !== 200) {
          alert("认证失败");
          return new Promise.reject(data);
        } else {
          const authInfo = {
            name: data.user.nickName,
            auth_id: data.user.userName,
            auth_secret: data.user.password,
          };
          sessionStorage.setItem("authInfo", JSON.stringify(authInfo));
        }
      })
      .catch((_) => {
        window.auth.logout();
      });
  }
})();
