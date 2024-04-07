import { Button, Form, Input, Space } from "antd";
import { mutate } from "swr";
import { SearchOutlined, PlusOutlined } from "@ant-design/icons";
import { useMainContext } from "@/contexts/main";
import * as DatasourceList from "@/api/data_source_list";

export default function HeaderForm() {
  const { dispatch, state } = useMainContext()!;
  const [form] = Form.useForm();

  async function onSearch() {
    const data = form.getFieldsValue(true);

    dispatch({
      type: "dataSourceList.setPagination",
      pagination: {
        ...state.dataSourceList.pagination,
        current: 1,
        data,
      },
    });
    await mutate([DatasourceList.LIST, state.dataSourceList.pagination]);
  }

  return (
    <Form form={form} name="basic" layout="inline">
      <Form.Item name="database_name">
        <Input placeholder="请输入名称" defaultValue={state.dataSourceList.pagination.data?.database_name!}/>
      </Form.Item>

      <Form.Item>
        <Space>
          <Button type="primary" icon={<SearchOutlined />} onClick={onSearch} />
          <Button
            type="primary"
            onClick={() => {
              dispatch({
                type: "dataSourceList.setEditFormOpen",
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
