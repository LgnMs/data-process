import { useSWRConfig } from "swr";
import {
  Button,
  Col,
  Drawer,
  Form,
  Input,
  message,
  Row,
  Select,
  Space,
} from "antd";
import React, { useEffect, useState } from "react";
import * as DatasourceList from "@/api/data_source_list";
import { ICommonDataSourceProps } from "@/app/manage/data-source-list/page";
import { useMainContext } from "@/contexts/main";
import { clone } from "lodash";

interface IEditFormProps extends ICommonDataSourceProps {
  open: boolean;
  close: () => void;
}
export default function EditForm(props: IEditFormProps) {
  const { mutate } = useSWRConfig();
  const [form] = Form.useForm();
  const { state, dispatch } = useMainContext()!;
  const [mode, setMode] = useState<"edit" | "add">("add");

  async function onSubmit() {
    await form.validateFields();
    const values = form.getFieldsValue(true);

    const data = {
      ...values,
    };

    let res;
    if (mode === "add") {
      res = await DatasourceList.add(data);
    } else {
      res = await DatasourceList.update_by_id(data.id, data);
    }

    if (res.data) {
      await mutate([DatasourceList.LIST, state.dataSourceList.pagination]);

      message.success("操作成功");

      close();
    }
  }

  function close() {
    props.close();
    form.resetFields();
    dispatch({
      type: "dataSourceList.setEditFormData",
      editFormData: null,
    });
  }

  useEffect(() => {
    if (state.dataSourceList.editFormOpen) {
      if (state.dataSourceList.editFormData) {
        const data: any = clone(state.dataSourceList.editFormData);


        form.setFieldsValue(data);
        setMode("edit");
      } else {
        setMode("add");
      }
    }
  }, [state.dataSourceList.editFormOpen]);

  return (
    <Drawer
      title={`${mode === "add" ? "新增" : "编辑"}数据源配置`}
      open={props.open}
      width={800}
      extra={
        <Space>
          <Button onClick={close}>取消</Button>
          <Button onClick={onSubmit} type="primary">
            提交
          </Button>
        </Space>
      }
      onClose={close}
    >
      <Form
        layout="vertical"
        form={form}
        labelAlign="left"
        labelWrap
        onFieldsChange={(changedFields) => {
          changedFields.forEach((item) => {});
        }}
      >
        <Row gutter={16}>
          <Col span={8}>
            <Form.Item
              label="数据库名称"
              name="database_name"
              rules={[{ required: true }]}
            >
              <Input placeholder="请输入" />
            </Form.Item>
          </Col>
          <Col span={16}>
            <Form.Item
              label="数据源名称"
              name="name"
            >
              <Input placeholder="请输入" />
            </Form.Item>
          </Col>
          <Col span={8}>
            <Form.Item
              label="数据库类型"
              name="database_type"
              rules={[{ required: true }]}
            >
              <Select
                placeholder="请输入"
                options={[
                  { label: "MYSQL", value: "MYSQL" },
                  { label: "POSTGRES", value: "POSTGRES" },
                  { label: "ORACLE", value: "ORACLE" },
                  { label: "人大金仓", value: "KINGBASE" },
                  { label: "MS SQL server", value: "MSSQL" },
                ]}
              />
            </Form.Item>
          </Col>
          <Col span={8}>
            <Form.Item label="模式" name="table_schema">
              <Input placeholder="public" />
            </Form.Item>
          </Col>
          <Col span={8}>
            <Form.Item label="Host" name="host" rules={[{ required: true }]}>
              <Input placeholder="127.0.0.1" />
            </Form.Item>
          </Col>
          <Col span={8}>
            <Form.Item label="端口" name="port" rules={[{ required: true }]}>
              <Input placeholder="3306" />
            </Form.Item>
          </Col>
          <Col span={8}>
            <Form.Item label="用户名" name="user" rules={[{ required: true }]}>
              <Input placeholder="请输入" />
            </Form.Item>
          </Col>
          <Col span={8}>
            <Form.Item
              label="密码"
              name="password"
              rules={[{ required: true }]}
            >
              <Input type="password" placeholder="请输入" />
            </Form.Item>
          </Col>
        </Row>
      </Form>
    </Drawer>
  );
}
