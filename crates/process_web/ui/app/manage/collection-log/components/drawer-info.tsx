import {
  Button,
  Drawer,
  Space,
} from "antd";
import React from "react";
import { ICommonCollectionSettingProps } from "@/app/manage/collection-setting/page";
import { useMainContext } from "@/contexts/main";

interface IEditFormProps extends ICommonCollectionSettingProps {
  open: boolean;
  close: () => void;
}
export default function DrawerInfo(props: IEditFormProps) {
  const { state, dispatch } = useMainContext()!;

  function close() {
    props.close();
    dispatch({
      type: "collectLog.setDrawerData",
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
      <code style={{whiteSpace: "pre-wrap"}}>
        {
          state.collectLog.drawerData?.running_log
        }
      </code>
    </Drawer>
  );
}
