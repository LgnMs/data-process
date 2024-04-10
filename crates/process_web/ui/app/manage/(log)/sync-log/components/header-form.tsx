import { Button, Form, Input, Space } from "antd";
import { SearchOutlined } from "@ant-design/icons";
import { mutate } from "swr";
import { useMainContext } from "@/contexts/main";
import * as SyncLog from "@/api/sync_log";

export default function HeaderForm() {
  const { dispatch, state } = useMainContext()!;
  const [form] = Form.useForm();

  async function onSearch() {
    const data = form.getFieldsValue(true);

    dispatch({
      type: "syncLog.setPagination",
      pagination: {
        ...state.syncLog.pagination,
        current: 1,
        data,
      },
    });
    await mutate([SyncLog.LIST, state.syncLog.pagination]);
  }

  return (
    <Form form={form} name="basic" layout="inline">
      <Form.Item name="sync_config_name">
        <Input placeholder="请输入同步任务名称" defaultValue={state.syncLog.pagination.data?.sync_config_name!}/>
      </Form.Item>

      <Form.Item>
        <Space>
          <Button type="primary" icon={<SearchOutlined />} onClick={onSearch} />
        </Space>
      </Form.Item>
    </Form>
  );
}
