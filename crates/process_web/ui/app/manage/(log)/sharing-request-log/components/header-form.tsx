import { Button, Form, Input, Space } from "antd";
import { SearchOutlined } from "@ant-design/icons";
import { mutate } from "swr";
import { useMainContext } from "@/contexts/main";
import * as SharingRequestLog from "@/api/sharing_request_log";

export default function HeaderForm() {
  const { dispatch, state } = useMainContext()!;
  const [form] = Form.useForm();

  async function onSearch() {
    const data = form.getFieldsValue(true);

    dispatch({
      type: "sharingRequestLog.setPagination",
      pagination: {
        ...state.sharingRequestLog.pagination,
        current: 1,
        data,
      },
    });
    await mutate([SharingRequestLog.LIST, state.sharingRequestLog.pagination]);
  }

  return (
    <Form form={form} name="basic" layout="inline">
      <Form.Item name="data_sharing_config_name">
        <Input placeholder="请输入共享配置名称" defaultValue={state.sharingRequestLog.pagination.data?.data_sharing_config_name!}/>
      </Form.Item>

      <Form.Item>
        <Space>
          <Button type="primary" icon={<SearchOutlined />} onClick={onSearch} />
        </Space>
      </Form.Item>
    </Form>
  );
}
