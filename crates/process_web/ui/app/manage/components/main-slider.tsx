import React, { ReactNode, useState } from "react";
import { Layout, Menu, Space } from "antd";
import { usePathname, useRouter } from "next/navigation";
import {
  CloudSyncOutlined,
  NodeCollapseOutlined,
  FileTextOutlined, DatabaseOutlined
} from "@ant-design/icons";

const { Sider } = Layout;
interface IMainSiderProps {
  onCollapse: (value: boolean) => void;
}

function MenuItem(props: { children: ReactNode; status?: string }) {
  return (
    <Space>
      {props.children} {props.status}
    </Space>
  );
}

const Menus = [
  {
    key: "/manage/data-source-list",
    icon: <DatabaseOutlined />,
    label: <MenuItem>数据源管理</MenuItem>,
  },
  {
    key: "/manage/collection-setting",
    icon: <NodeCollapseOutlined />,
    label: <MenuItem>采集任务</MenuItem>,
  },
  {
    key: "/manage/sync-setting",
    icon: <CloudSyncOutlined />,
    label: <MenuItem>同步任务</MenuItem>,
  },
  {
    key: "/manage",
    icon: <FileTextOutlined />,
    label: <MenuItem>日志</MenuItem>,
    children: [
      {
        key: "collection-log",
          label: <MenuItem>采集任务日志</MenuItem>,
      },
      {
        key: "sync-log",
        label: <MenuItem>同步任务日志</MenuItem>,
      },
    ],
  },
];

export function MainSlider(props: IMainSiderProps) {
  const [collapsed, setCollapsed] = useState(false);
  const router = useRouter();
  const pathname = usePathname();

  function getKey(list: Array<any>, arr?: Array<any>, parentKey?: string) {
    const keys = arr ? arr : [];

    for (let i = 0; i < list.length; i += 1) {
      let item = list[i];
      if (item.key === pathname || `${parentKey}/${item.key}` === pathname) {
        keys.push(item.key);
        return keys;
      } else {
        if (item.children) {
          keys.push(item.key);
          getKey(item.children, keys, item.key);
        }
      }
    }
    return keys;
  }

  const keys = getKey(Menus);

  // /**
  //  * 权限过滤
  //  * @param menus 菜单
  //  * @param permissions 权限列表
  //  * @param pre 权限的前缀scope 例如：om:shu-ju-jian-kong 其中om:是前缀
  //  * @returns 有权限的菜单列表
  //  */
  // function filterMenuByPermission(menus: typeof Menus, pre: string, permissions?: string[], ) {
  //   let list: typeof Menus = []
  //   if (permissions) {
  //     if (permissions.includes('*:*:*')) return menus;

  //     list = menus.filter(menu => {
  //       if (menu.children) {
  //         menu.children = filterMenuByPermission(menu.children as any, pre, permissions)
  //         return menu.children.length > 0
  //       } else {
  //         return permissions.find(o => o === pre + menu.key)
  //       }
  //     })
  //   }
  //   return list
  // }

  const onSelect = (value: any) => {
    router.push(value.key);
  };
  console.log(keys)

  return (
    <Sider
      width={200}
      theme="light"
      style={{
        position: "fixed",
        left: 0,
        top: "64px",
        height: "calc(100vh - 64px)",
        overflow: "auto",
      }}
      breakpoint="xxl"
      collapsible
      collapsed={collapsed}
      onCollapse={(value) => {
        setCollapsed(value);
        props.onCollapse(value);
      }}
    >
      {/* { state.userInfo
      ? <Menu
          mode="inline"
          onSelect={onSelect}
          defaultSelectedKeys={[keys[keys.length - 1]]}
          defaultOpenKeys={keys.slice(0, keys.length - 1)}
          style={{ height: '100%', borderRight: 0 }}
          items={filterMenuByPermission(Menus, 'om:', state.permissions)}
        />
      : <Spin />
     } */}
      <Menu
        mode="inline"
        onSelect={onSelect}
        defaultSelectedKeys={[keys[keys.length - 1]]}
        defaultOpenKeys={keys}
        style={{ height: "100%", borderRight: 0 }}
        items={Menus}
      />
    </Sider>
  );
}
