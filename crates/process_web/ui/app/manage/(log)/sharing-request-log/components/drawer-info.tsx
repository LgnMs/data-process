import { Button, Drawer, Space } from "antd";
import React from "react";
import { ICommonSyncSettingProps } from "@/app/manage/sync-setting/page";
import { useMainContext } from "@/contexts/main";

interface IEditFormProps extends ICommonSyncSettingProps {
  open: boolean;
  close: () => void;
}
export default function DrawerInfo(props: IEditFormProps) {
  const { state, dispatch } = useMainContext()!;

  function close() {
    props.close();
    dispatch({
      type: "sharingRequestLog.setDrawerData",
      drawerData: null,
    });
  }

  return (
    <Drawer
      title={`查看日志`}
      open={props.open}
      width={800}
      extra={
        <Space>
          <Button onClick={close}>取消</Button>
        </Space>
      }
      onClose={close}
    >
      <code style={{ whiteSpace: "pre-wrap" }}>
        {state.sharingRequestLog.drawerData?.log}
      </code>
    </Drawer>
  );
}
