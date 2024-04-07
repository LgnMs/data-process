import { Button, Form, Input, Space } from "antd";
import { mutate } from "swr";
import { SearchOutlined, PlusOutlined } from "@ant-design/icons";
import { useMainContext } from "@/contexts/main";
import * as DataSharingConfig from "@/api/data_sharing_config";

export default function HeaderForm() {
  const { dispatch, state } = useMainContext()!;
  const [form] = Form.useForm();

  async function onSearch() {
    const data = form.getFieldsValue(true);

    dispatch({
      type: "dataSharingConfig.setPagination",
      pagination: {
        ...state.dataSharingConfig.pagination,
        current: 1,
        data,
      },
    });
    await mutate([DataSharingConfig.LIST, state.dataSharingConfig.pagination]);
  }

  return (
    <Form form={form} name="basic" layout="inline">
      <Form.Item name="name">
        <Input placeholder="请输入名称" defaultValue={state.dataSharingConfig.pagination.data?.name!}/>
      </Form.Item>

      <Form.Item>
        <Space>
          <Button type="primary" icon={<SearchOutlined />} onClick={onSearch} />
          <Button
            type="primary"
            onClick={() => {
              dispatch({
                type: "dataSharingConfig.setEditFormOpen",
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
