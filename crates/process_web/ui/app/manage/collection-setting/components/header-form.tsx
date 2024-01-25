import {Button, Form, Input, Space} from "antd";
import { SearchOutlined, PlusOutlined } from "@ant-design/icons";
import {useState} from "react";
import EditForm from "@/app/manage/collection-setting/components/edit-form";
import {ICommonCollectionSettingProps} from "@/app/manage/collection-setting/page";
import { useMainContext } from "@/contexts/main";

interface IHeaderFormProps extends ICommonCollectionSettingProps {}

export default function HeaderForm() {

  const { dispatch } = useMainContext()!

  return (
    <Form name="basic" layout="inline">
      <Form.Item name="enterpriseName">
        <Input placeholder="请输入名称" />
      </Form.Item>

      <Form.Item>
        <Space>
          <Button
            type="primary"
            icon={<SearchOutlined rev={undefined} />}
          />
          <Button
            type="primary"
            onClick={() => {
              dispatch({
                type: 'collectConfig.setEditFormOpen',
                editFormOpen: true
              });
            }}
            icon={<PlusOutlined rev={undefined} />}
          />

        </Space>
      </Form.Item>
    </Form>
  );
}
