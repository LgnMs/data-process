import useSWR, { useSWRConfig } from "swr";
import {
  Button,
  Col,
  Drawer,
  Form,
  Input,
  message,
  Radio,
  Row, Select,
  Space
} from "antd";
import React, { useEffect, useState } from "react";
import * as SyncConfig from "@/api/sync_config";
import * as DataSourceList from "@/api/data_source_list";
import { DataSourceList as IDataSourceList } from "@/api/models/DataSourceList"
import { ICommonCollectionSettingProps } from "@/app/manage/collection-setting/page";
import { useMainContext } from "@/contexts/main";
import { clone } from "lodash";
import CronEdit from "@/app/manage/components/cron-edit";

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
          <Button onClick={() => {
            console.log(form.getFieldsValue())
          }} type="primary">
            de
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
              label="源表数据库"
              name="data_source"
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

          <Col span={8}>
            <Form.Item
              label="源表列"
              name="source_table_columns"
              rules={[{ required: true }]}
            >
              <Input placeholder="请输入" />
            </Form.Item>
          </Col>
          <Col span={24}>
            <Form.Item
              label="源表查询sql"
              name="query_sql"
              rules={[{ required: true }]}
            >
              <Input.TextArea placeholder="请输入" />
            </Form.Item>
          </Col>
          <Col span={8}>
            <Form.Item
              label="目标表数据库"
              name="target_data_source"
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
              label="目标表查询sql模板"
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

function DataSourceSelect(props: {
  value?: IDataSourceList
  onChange?: (data: IDataSourceList) => void
}) {
  const pagination = {
      current: 1,
      page_size: 999,
      data: null,
  };
  const { data,  } = useSWR(
      [DataSourceList.LIST, pagination],
      ([_, pagination]) => DataSourceList.list(pagination)
  );

  let options: any[] = [];

  if (data?.data) {
    options = data.data.list.map(item => {
      return {
        value: item.id,
        label: item.database_name
      }
    })
  }
  return <Select allowClear placeholder="请选择" value={props.value?.id}  options={options} onChange={(value) => {
    data?.data?.list.forEach(item => {
      if (value === item.id) {
        props.onChange?.(item)
      }
    })
  }} />
}