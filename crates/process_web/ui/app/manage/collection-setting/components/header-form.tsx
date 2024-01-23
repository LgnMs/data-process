import { Button, Form, Input, Space } from "antd";
import { SearchOutlined, PlusOutlined } from "@ant-design/icons";
import { useSWRConfig } from "swr";
import * as CollectConfig from "@/api/collect_config";

export default function HeaderForm(props: {
  onAddClick: () => void;
  onSearch: (name: string) => void;
}) {
  const [form] = Form.useForm();
  const { mutate } = useSWRConfig();

  return (
    <Form form={form} name="basic" layout="inline">
      <Form.Item name="enterpriseName">
        <Input placeholder="请输入名称" />
      </Form.Item>

      <Form.Item>
        <Space>
          <Button
            type="primary"
            onClick={() => props.onSearch(form.getFieldValue("enterpriseName"))}
            icon={<SearchOutlined rev={undefined} />}
          />
          <Button
            type="primary"
            onClick={async () => {
              const res = await CollectConfig.add({
                body: null,
                cache_table_name: null,
                current_key: null,
                desc: null,
                headers: undefined,
                loop_request_by_pagination: null,
                map_rules: [["a", "b"]],
                method: "",
                name: "123",
                page_size_key: null,
                template_string: "",
                url: "",
              });
              if (res.data) {
                mutate(CollectConfig.LIST);
              }
            }}
            icon={<PlusOutlined rev={undefined} />}
          />
        </Space>
      </Form.Item>
    </Form>
  );
}
