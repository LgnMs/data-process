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
        data,
      },
    });
    await mutate([SyncLog.LIST, state.syncLog.pagination]);
  }

  return (
    <Form form={form} name="basic" layout="inline">
      <Form.Item name="name">
        <Input placeholder="请输入采集配置名称" />
      </Form.Item>

      <Form.Item>
        <Space>
          <Button type="primary" icon={<SearchOutlined />} onClick={onSearch} />
        </Space>
      </Form.Item>
    </Form>
  );
}
