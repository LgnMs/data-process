import { Button, Form, Input, Space } from "antd";
import { SearchOutlined } from "@ant-design/icons";
import { mutate } from "swr";
import { useMainContext } from "@/contexts/main";
import * as CollectLog from "@/api/collect_log";

export default function HeaderForm() {
  const { dispatch, state } = useMainContext()!;
  const [form] = Form.useForm();

  async function onSearch() {
    const data = form.getFieldsValue(true);

    dispatch({
      type: "collectLog.setPagination",
      pagination: {
        ...state.collectLog.pagination,
        current: 1,
        data,
      },
    });
    await mutate([CollectLog.LIST, state.collectLog.pagination]);
  }

  return (
    <Form form={form} name="basic" layout="inline">
      <Form.Item name="collect_config_name">
        <Input placeholder="请输入采集任务名称" defaultValue={state.collectLog.pagination.data?.collect_config_name!} />
      </Form.Item>

      <Form.Item>
        <Space>
          <Button type="primary" icon={<SearchOutlined />} onClick={onSearch} />
        </Space>
      </Form.Item>
    </Form>
  );
}
