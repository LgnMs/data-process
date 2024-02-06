import { Button, Form, Input, Space } from "antd";
import { mutate } from "swr";
import { SearchOutlined, PlusOutlined } from "@ant-design/icons";
import { useMainContext } from "@/contexts/main";
import * as CollectConfig from "@/api/collect_config";

export default function HeaderForm() {
  const { dispatch, state } = useMainContext()!;
  const [form] = Form.useForm();

  async function onSearch() {
    const data = form.getFieldsValue(true);

    dispatch({
      type: "collectConfig.setPagination",
      pagination: {
        ...state.collectConfig.pagination,
        data
      }
    })
    await mutate([CollectConfig.LIST, state.collectConfig.pagination]);
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
                type: "collectConfig.setEditFormOpen",
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
