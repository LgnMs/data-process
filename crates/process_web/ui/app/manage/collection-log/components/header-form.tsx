import { Button, Form, Input, Space } from "antd";
import { SearchOutlined } from "@ant-design/icons";
import { useMainContext } from "@/contexts/main";

export default function HeaderForm() {

  return (
    <Form name="basic" layout="inline">
      <Form.Item name="name">
        <Input placeholder="请输入名称" />
      </Form.Item>

      <Form.Item>
        <Space>
          <Button type="primary" icon={<SearchOutlined />} />
        </Space>
      </Form.Item>
    </Form>
  );
}
