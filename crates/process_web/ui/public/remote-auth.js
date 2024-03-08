// 当USE_REMOTE_AUTH=true时加载这个脚本
(() => {
  const USER_ADMIN_HOST = "http://127.0.0.1:8000"
  function hasToken() {
    return window.sessionStorage.getItem("token")
  }

  window.auth = {
    login() {
      if (typeof window === "undefined") {
        return;
      }
      const { location } = window;
      const originUrl = `${location.origin}${location.pathname === '/' ? '' : location.pathname}`;
      window.location.href = `${USER_ADMIN_HOST}/login?remoteUrl=${originUrl}`;
    },
    logout() {
      const { location } = window;
      const originUrl = `${location.origin}${location.pathname === '/' ? '' : location.pathname}`;
      window.sessionStorage.removeItem("token")
      window.location.href = `${USER_ADMIN_HOST}/logout?remoteUrl=${originUrl}`;
    }
  }


  if (!hasToken()) {
    window.auth.login();
  } else {
    const windowUrlParams = new URLSearchParams(params.toString())
    const token = windowUrlParams.get('token');
    if (token) {
      window.sessionStorage.setItem('token', token)
      windowUrlParams.delete('token')
      window.history.replaceState({}, "", window.location.origin + window.location.pathname + windowUrlParams.toString())
    }
    fetch('http://xxx:xx/getInfo')
      .then(res => {
        return res.json();
      })
      .then(data => {
        console.log(data)
      })
      .catch(_ => {
        window.auth.logout()
      })

  }

})()