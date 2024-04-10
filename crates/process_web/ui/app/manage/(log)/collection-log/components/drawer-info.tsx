import { Button, Drawer, Space, Spin } from "antd";
import React from "react";
import { ICommonCollectionSettingProps } from "@/app/manage/collection-setting/page";
import { useMainContext } from "@/contexts/main";
import useSWR from "swr";
import { FIND_BY_ID, find_by_id } from "@/api/collect_log";

interface IEditFormProps extends ICommonCollectionSettingProps {
  open: boolean;
  close: () => void;
}
export default function DrawerInfo(props: IEditFormProps) {
  const { state, dispatch } = useMainContext()!;

  if (!state.collectLog.drawerData || !state.collectLog.drawerData.id) return
  const { data, isLoading } = useSWR([FIND_BY_ID, state.collectLog.drawerData.id], ([_, id]) => find_by_id(id))

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
      { isLoading && <Spin />}
      <code style={{ whiteSpace: "pre-wrap" }}>
        {data?.data?.running_log}
      </code>
    </Drawer>
  );
}
