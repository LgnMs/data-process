import { Button, Form, Input, Space } from "antd";
import { SearchOutlined, PlusOutlined } from '@ant-design/icons'


export default function HeaderForm(props: { onAddClick: () => void; onSearch: (name: string) => void }) {
  const [form] = Form.useForm();

  return <Form form={form} name="basic" layout="inline">

    <Form.Item name="enterpriseName">
      <Input placeholder="请输入名称"/>
    </Form.Item>

    <Form.Item>
      <Space>
        <Button type="primary" onClick={() => props.onSearch(form.getFieldValue('enterpriseName'))}  icon={<SearchOutlined rev={undefined} />} />
        <Button type="primary" onClick={() => props.onAddClick()}  icon={<PlusOutlined rev={undefined} />} />
      </Space>
    </Form.Item>
  </Form>
}