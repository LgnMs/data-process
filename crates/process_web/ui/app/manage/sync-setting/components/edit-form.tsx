import useSWR, { useSWRConfig } from "swr";
import {
  Button,
  Col,
  Drawer,
  Form,
  FormInstance,
  Input,
  message,
  Radio,
  Row,
  Select,
  Space,
} from "antd";
import React, { useEffect, useState } from "react";
import * as SyncConfig from "@/api/sync_config";
import * as DataSourceList from "@/api/data_source_list";
import { ICommonCollectionSettingProps } from "@/app/manage/collection-setting/page";
import { useMainContext } from "@/contexts/main";
import { clone } from "lodash";
import CronEdit from "@/app/manage/components/cron-edit";
import LabelTips from "@/app/manage/components/label-tips";

interface IEditFormProps extends ICommonCollectionSettingProps {
  open: boolean;
  close: () => void;
}
export default function EditForm(props: IEditFormProps) {
  const { mutate } = useSWRConfig();
  const [form] = Form.useForm();
  const { state, dispatch } = useMainContext()!;
  const [mode, setMode] = useState<"edit" | "add">("add");
  const [autoExec, setAutoExec] = useState(0);
  const [genLoading, setGenLoading] = useState(false);
  const [gen2Loading, setGen2Loading] = useState(false);

  async function onSubmit() {
    await form.validateFields();
    const values = form.getFieldsValue(true);

    const data = {
      ...values,
    };

    let res;
    if (mode === "add") {
      res = await SyncConfig.add(data);
    } else {
      res = await SyncConfig.update_by_id(data.id, data);
    }

    if (res.data) {
      await mutate([SyncConfig.LIST, state.syncConfig.pagination]);

      message.success("操作成功");

      close();
    }
  }

  function close() {
    props.close();
    form.resetFields();
    dispatch({
      type: "syncConfig.setEditFormData",
      editFormData: null,
    });
  }

  useEffect(() => {
    if (state.syncConfig.editFormOpen) {
      if (state.syncConfig.editFormData) {
        const data: any = clone(state.syncConfig.editFormData);

        if (data.cron) {
          setAutoExec(1);
        } else {
          setAutoExec(0);
        }

        form.setFieldsValue(data);
        setMode("edit");
      } else {
        setMode("add");
      }
    }
  }, [state.syncConfig.editFormOpen]);

  async function generateSourceSql() {
    const sql = await generateQuerySql(
      form,
      "data_source_id",
      "source_table_name"
    );
    form.setFieldValue("query_sql", sql);
  }

  async function generateTargetSql() {
    const data_source_id = form.getFieldValue("target_data_source_id")!;
    const data_source = (await DataSourceList.find_by_id(data_source_id)).data;
    const source_table_name = form.getFieldValue("target_table_name")!;
    const table_schema = data_source?.table_schema;
    const res = await DataSourceList.query_table_columns({
      data_source,
      table_name: source_table_name,
    });
    let sql = "INSERT INTO ";
    let cols: string[] = [];
    res.data?.forEach((item) => {
      for (const key in item) {
        cols.push(item[key]);
      }
    });
    const table_name = table_schema
      ? `${table_schema}.${source_table_name}`
      : source_table_name;
    //INSERT INTO public.sync_test_table (code, naem) VALUES('${code}', '${name}');
    sql += `${table_name} (${cols.join(", ")}) VALUES();`;

    form.setFieldValue("target_query_sql_template", sql);
  }

  return (
    <Drawer
      title={`${mode === "add" ? "新增" : "编辑"}同步配置`}
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
          changedFields.forEach((_) => {});
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
              label="源表数据库"
              name="data_source_id"
              rules={[{ required: true }]}
            >
              {/*<Input placeholder="请输入" />*/}
              <DataSourceSelect />
            </Form.Item>
          </Col>
          <Col span={8}>
            <Form.Item
              label="源表"
              name="source_table_name"
              rules={[{ required: true }]}
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
                    onClick={async () => {
                      setGenLoading(true);
                      await generateSourceSql();
                      setGenLoading(false);
                    }}
                    loading={genLoading}
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
          <Col span={8}>
            <Form.Item
              label="目标表数据库"
              name="target_data_source_id"
              rules={[{ required: true }]}
            >
              <DataSourceSelect />
            </Form.Item>
          </Col>
          <Col span={8}>
            <Form.Item
              label="目标表"
              name="target_table_name"
              rules={[{ required: true }]}
            >
              <Input placeholder="请输入" />
            </Form.Item>
          </Col>
          <Col span={24}>
            <Form.Item
              label={
                <LabelTips tips="例如：INSERT INTO public.sync_test_table (code, naem) VALUES('${code}', '${name}');">
                  目标表查询sql模板&nbsp;
                  <Button
                    type="primary"
                    size="small"
                    onClick={async () => {
                      setGen2Loading(true);
                      await generateTargetSql();
                      setGen2Loading(false);
                    }}
                    loading={gen2Loading}
                  >
                    点击生成
                  </Button>
                </LabelTips>
              }
              name="target_query_sql_template"
              rules={[{ required: true }]}
            >
              <Input.TextArea placeholder="请输入" />
            </Form.Item>
          </Col>

          <Col span={24}>
            <Form.Item
              label={
                <div>
                  执行周期&nbsp;
                  <Radio.Group
                    value={autoExec}
                    onChange={(e) => {
                      if (e.target.value === 0) {
                        form.setFieldValue("cron", null);
                      } else {
                        form.setFieldValue("cron", "0 0 1 * *");
                      }
                      setAutoExec(e.target.value);
                    }}
                  >
                    <Radio value={0}>停用</Radio>
                    <Radio value={1}>启用</Radio>
                  </Radio.Group>
                </div>
              }
              name={autoExec === 1 ? "cron" : undefined}
            >
              {autoExec === 1 && <CronEdit />}
            </Form.Item>
          </Col>
        </Row>
      </Form>
    </Drawer>
  );
}

export function DataSourceSelect(props: {
  value?: number;
  onChange?: (data?: number) => void;
}) {
  const pagination = {
    current: 1,
    page_size: 999,
    data: null,
  };
  const { data } = useSWR(
    [DataSourceList.LIST, pagination],
    ([_, pagination]) => DataSourceList.list(pagination)
  );

  let options: any[] = [];

  if (data?.data) {
    options = data.data.list.map((item) => {
      return {
        value: item.id,
        label: item.name || item.database_name,
      };
    });
  }
  return (
    <Select
      allowClear
      placeholder="请选择"
      value={props.value}
      options={options}
      onChange={(value) => {
        props.onChange?.(value);
      }}
      onClear={() => {
        props.onChange?.();
      }}
    />
  );
}

export async function getSourceDataBaseColumns(
  form: FormInstance,
  data_source_filed: string,
  table_name_filed: string
) {
  const data_source_id = form.getFieldValue(data_source_filed)!;
  const source_table_name = form.getFieldValue(table_name_filed)!;
  const data_source = (await DataSourceList.find_by_id(data_source_id)).data;
  const res = await DataSourceList.query_table_columns({
    data_source,
    table_name: source_table_name,
  });
  let cols: string[] = [];
  res.data?.forEach((item) => {
    for (const key in item) {
      cols.push(item[key]);
    }
  });
  return cols;
}

export async function generateQuerySql(
  form: FormInstance,
  data_source_filed: string,
  table_name_filed: string
) {
  const data_source_id = form.getFieldValue(data_source_filed)!;
  const source_table_name = form.getFieldValue(table_name_filed)!;
  const data_source = (await DataSourceList.find_by_id(data_source_id)).data;
  const table_schema = data_source?.table_schema;
  let sql = "SELECT ";
  let cols: string[] = await getSourceDataBaseColumns(
    form,
    data_source_filed,
    table_name_filed
  );

  const table_name = table_schema
    ? `${table_schema}.${source_table_name}`
    : source_table_name;
  sql += `${cols.join(", ")} FROM ${table_name};`;

  return sql;
}
