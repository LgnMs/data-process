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
import * as DataSharingConfig from "@/api/data_sharing_config";
import { ICommonDataSharingConfigProps } from "@/app/manage/data-sharing-config/page";
import { useMainContext } from "@/contexts/main";
import { clone } from "lodash";
import { DataSourceSelect, generateQuerySql } from "@/app/manage/sync-setting/components/edit-form";
import LabelTips from "@/app/manage/components/label-tips";

interface IEditFormProps extends ICommonDataSharingConfigProps {
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
      res = await DataSharingConfig.add(data);
    } else {
      res = await DataSharingConfig.update_by_id(data.id, data);
    }

    if (res.data) {
      await mutate([
        DataSharingConfig.LIST,
        state.dataSharingConfig.pagination,
      ]);

      message.success("操作成功");

      close();
    }
  }

  function close() {
    props.close();
    form.resetFields();
    dispatch({
      type: "dataSharingConfig.setEditFormData",
      editFormData: null,
    });
  }

  async function generateSourceSql() {
    const sql = await generateQuerySql(form, "data_source", "table_name");
    form.setFieldValue("query_sql", sql);
  }

  useEffect(() => {
    if (state.dataSharingConfig.editFormOpen) {
      if (state.dataSharingConfig.editFormData) {
        const data: any = clone(state.dataSharingConfig.editFormData);

        form.setFieldsValue(data);
        setMode("edit");
      } else {
        setMode("add");
      }
    }
  }, [state.dataSharingConfig.editFormOpen]);

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
            <Form.Item label="名称" name="name" rules={[{ required: true }]}>
              <Input placeholder="请输入" />
            </Form.Item>
          </Col>
          <Col span={8}>
            <Form.Item
              label="查询数据源"
              name="data_source"
              rules={[{ required: true }]}
            >
              <DataSourceSelect />
            </Form.Item>
          </Col>
          <Col span={8}>
            <Form.Item
              label="表名"
              name="table_name"
            >
              <Input placeholder="请输入" />
            </Form.Item>
          </Col>
          <Col span={24}>
            <Form.Item
              label={
                <LabelTips
                  tips={`例如：SELECT id, parent_code, parent_ci_id, code, "name", unit, value, ci_id, type_name FROM test_data;"`}
                >
                  源表查询sql&nbsp;
                  <Button
                    type="primary"
                    size="small"
                    onClick={generateSourceSql}
                  >
                    点击生成
                  </Button>
                </LabelTips>
              }
              name="query_sql"
              rules={[{ required: true }]}
            >
              <Input.TextArea placeholder="请输入" />
            </Form.Item>
          </Col>
        </Row>
      </Form>
    </Drawer>
  );
}
