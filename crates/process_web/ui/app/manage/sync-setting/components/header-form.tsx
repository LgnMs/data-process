import { Button, Form, Input, Space } from "antd";
import { mutate } from "swr";
import { SearchOutlined, PlusOutlined } from "@ant-design/icons";
import { useMainContext } from "@/contexts/main";
import * as SyncConfig from "@/api/sync_config";

export default function HeaderForm() {
  const { dispatch, state } = useMainContext()!;
  const [form] = Form.useForm();

  async function onSearch() {
    const data = form.getFieldsValue(true);

    dispatch({
      type: "syncConfig.setPagination",
      pagination: {
        ...state.syncConfig.pagination,
        data,
      },
    });
    await mutate([SyncConfig.LIST, state.syncConfig.pagination]);
  }

  return (
    <Form form={form} name="basic" layout="inline">
      <Form.Item name="name">
        <Input placeholder="请输入名称" />
      </Form.Item>

      <Form.Item>
        <Space>
          <Button type="primary" icon={<SearchOutlined />} onClick={onSearch} />
          <Button
            type="primary"
            onClick={() => {
              dispatch({
                type: "syncConfig.setEditFormOpen",
                editFormOpen: true,
              });
            }}
            icon={<PlusOutlined />}
          />
        </Space>
      </Form.Item>
    </Form>
  );
}
