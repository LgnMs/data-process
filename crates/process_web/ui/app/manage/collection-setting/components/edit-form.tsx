import { useSWRConfig } from "swr";
import {
  Button,
  Col,
  Drawer,
  Form,
  Input,
  InputNumber,
  message,
  Radio,
  Row,
  Select,
  Space,
  Modal,
} from "antd";
import React, { useEffect, useState } from "react";
import { CloseOutlined, PlusOutlined } from "@ant-design/icons";
import LabelTips from "@/app/manage/components/label-tips";
import * as CollectConfig from "@/api/collect_config";
import { ICommonCollectionSettingProps } from "@/app/manage/collection-setting/page";
import { useMainContext } from "@/contexts/main";
import { chunk, clone } from "lodash";
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
  const [shouldUpdateSql, setShouldUpdateSql] = useState(false);
  const [autoExec, setAutoExec] = useState(0);

  async function onSubmit() {
    if (shouldUpdateSql) {
      await new Promise((resolve) => {
        Modal.confirm({
          title: "配置发生了变化，是否重新生成SQL",
          onOk() {
            generateSql();
            resolve(true);
          },
          onCancel() {
            resolve(true);
          },
        });
      });
    }

    await form.validateFields();
    const values = form.getFieldsValue(true);

    const headers: Record<string, string> = {};

    values.headers?.forEach((item: any) => {
      headers[item.key] = item.value;
    });

    const nested_config = values.nested_config?.map((item: any) => {
      return {
        root_key: item.key,
        children_key: item.value,
        id_key: item.value2,
      };
    });

    const map_rules = values.map_rules?.map((item: any) => [
      item.key,
      item.value,
    ]);
    const [db_columns_config, db_columns_config2] = chunk(values.db_columns_config, Math.round(values.db_columns_config.length / 2))

    const body: Record<string, string | number | boolean> = {};
    values.body?.forEach((item: any) => {
      if (Number.isNaN(Number(item.value))) {
        if (item.value === "true") {
          body[item.key] = true;
        } else if (item.value === "false") {
          body[item.key] = false;
        } else {
          body[item.key] = item.value;
        }
      } else {
        body[item.key] = Number(item.value);
      }
    });

    const data = {
      ...values,
      db_columns_config,
      db_columns_config2,
      headers,
      map_rules,
      nested_config,
      body: JSON.stringify(body),
    };

    let res;
    if (mode === "add") {
      res = await CollectConfig.add(data);
    } else {
      res = await CollectConfig.update_by_id(data.id, data);
    }

    if (res?.data) {
      await mutate([CollectConfig.LIST, state.collectConfig.pagination]);

      message.success("操作成功");

      close();
    }
  }

  function close() {
    props.close();
    form.resetFields();
    dispatch({
      type: "collectConfig.setEditFormData",
      editFormData: null,
    });
  }

  useEffect(() => {
    setShouldUpdateSql(false);
    if (state.collectConfig.editFormOpen) {
      if (state.collectConfig.editFormData) {
        const data: any = clone(state.collectConfig.editFormData);

        if (data.headers) {
          data.headers = Object.keys(data.headers).map((key) => {
            return { key, value: data.headers[key] };
          });
        }
        if (data.body) {
          const obj = JSON.parse(data.body);
          data.body = Object.keys(obj).map((key) => {
            return { key, value: obj[key] };
          });
        }
        if (data.nested_config) {
          data.nested_config = data.nested_config.map((item: any) => {
            return {
              key: item.root_key,
              value: item.children_key,
              value2: item.id_key,
            };
          });
        }
        if (data.map_rules) {
          data.map_rules = data.map_rules.map((item: Array<string>) => {
            return { key: item[0], value: item[1] };
          });
        }

        let arr: any[] = [];
        if (data.db_columns_config) {
          arr = arr.concat(data.db_columns_config)
        }
        if (data.db_columns_config2) {
          arr = arr.concat(data.db_columns_config2)
        }
        data.db_columns_config = arr;

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
  }, [state.collectConfig.editFormOpen]);

  function setDefaultForPostHeaders(method: "GET" | "POST") {
    let headers: Array<{ key: string; value: string }> | undefined =
      form.getFieldValue("headers");
    if (headers === undefined) {
      headers = [];
    }

    let jsonHeaderIndex = -1;
    jsonHeaderIndex = headers?.findIndex(
      (header) =>
        header.key === "Content-Type" && header.value === "application/json"
    );

    if (jsonHeaderIndex === -1 && method === "POST") {
      headers.push({ key: "Content-Type", value: "application/json" });
    }

    if (jsonHeaderIndex > -1 && method === "GET") {
      headers.splice(jsonHeaderIndex, 1);
    }

    form.setFieldValue("headers", headers);
  }

  async function generateSql() {
    if (shouldUpdateSql) {
      setShouldUpdateSql(false);
    }
    const db_columns_config = form.getFieldValue("db_columns_config");
    const cache_table_name = form.getFieldValue("cache_table_name");
    if (!db_columns_config) {
      await message.error("请先配置列配置");
      return;
    }
    if (!cache_table_name) {
      await message.error("请先配置缓存表");
      return;
    }
    let template_str = `INSERT INTO ${cache_table_name}`;
    const columns: string[] = [];
    const columns_value: string[] = [];
    db_columns_config.forEach(
      (column: { key: string; value: string; type: string }) => {
        columns.push(column.key);
        columns_value.push(`'\${${column.value}}'`);
      }
    );
    template_str += ` (${columns.join(", ")}) VALUES (${columns_value.join(
      ", "
    )})`;

    form.setFieldValue("template_string", template_str);
  }

  function FormArrayList(props: {
    name: string;
    buttonText?: string;
    initialValue?: any[];
    isColumnConfig?: boolean;
    isNestedConfig?: boolean;
  }) {
    return (
      <Form.List name={props.name} initialValue={props.initialValue}>
        {(fields, { add, remove }, { errors }) => {
          return (
            <div
              style={{ display: "flex", flexDirection: "column", rowGap: 16 }}
            >
              {fields.map((field) => (
                <Space key={field.key}>
                  <Form.Item noStyle name={[field.name, "key"]}>
                    <Input placeholder="key" />
                  </Form.Item>
                  <Form.Item noStyle name={[field.name, "value"]}>
                    <Input placeholder="value" />
                  </Form.Item>
                  {props.isNestedConfig && (
                    <Form.Item noStyle name={[field.name, "value2"]}>
                      <Input placeholder="value2" />
                    </Form.Item>
                  )}
                  {props.isColumnConfig && (
                    <Form.Item
                      noStyle
                      name={[field.name, "type"]}
                      initialValue="varchar"
                    >
                      <Select
                        options={[
                          { value: "varchar", label: "字符串" },
                          { value: "integer", label: "数字" },
                          { value: "timestamp", label: "日期" },
                          { value: "boolean", label: "布尔 boolean" },
                          {
                            value: "double precision",
                            label: "双精度 double precision",
                          },
                          { value: "text", label: "文本 text" },
                          { value: "bigint", label: "bigint" },
                        ]}
                      ></Select>
                    </Form.Item>
                  )}
                  <CloseOutlined
                    onClick={() => {
                      remove(field.name);
                      setShouldUpdateSql(true);
                    }}
                    rev={undefined}
                  />
                </Space>
              ))}
              <Button
                type="dashed"
                onClick={() => {
                  add();
                  setShouldUpdateSql(true);
                }}
                style={{ width: "380px" }}
                icon={<PlusOutlined rev={undefined} />}
              >
                {props.buttonText ? props.buttonText : "添加"}
              </Button>
              <Form.ErrorList errors={errors} />
            </div>
          );
        }}
      </Form.List>
    );
  }

  return (
    <Drawer
      title={`${mode === "add" ? "新增" : "编辑"}采集配置`}
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
          changedFields.forEach((item) => {
            if (item.name[0] === "method") {
              setDefaultForPostHeaders(item.value);
            }
            if (
              item.name[0] === "db_columns_config" ||
              item.name[0] === "cache_table_name"
            ) {
              setShouldUpdateSql(true);
            }
          });
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
              label="请求类型"
              name="method"
              rules={[{ required: true }]}
            >
              <Radio.Group>
                <Radio value="GET">GET</Radio>
                <Radio value="POST">POST</Radio>
              </Radio.Group>
            </Form.Item>
          </Col>
          <Col span={8}>
            <Form.Item
              label="数据暂存表"
              name="cache_table_name"
              rules={[{ required: true }]}
            >
              <Input placeholder="请输入" />
            </Form.Item>
          </Col>
          <Col span={24}>
            <Form.Item
              label="请求地址(url)"
              name="url"
              rules={[{ required: true }]}
            >
              <Input placeholder="请输入" />
            </Form.Item>
          </Col>
          <Col span={8}>
            <Form.Item
              label={
                <LabelTips tips="指定页码和分页大小key后，会">
                  是否循环请求
                </LabelTips>
              }
              name="loop_request_by_pagination"
            >
              <Radio.Group>
                <Radio value={false}>否</Radio>
                <Radio value={true}>是</Radio>
              </Radio.Group>
            </Form.Item>
          </Col>
          <Form.Item
            noStyle
            shouldUpdate={(prevValues: any, currentValues: any) =>
              prevValues["loop_request_by_pagination"] !==
              currentValues["loop_request_by_pagination"]
            }
          >
            {({ getFieldValue }) =>
              getFieldValue("loop_request_by_pagination") === true ? (
                <>
                  <Col span={8}>
                    <Form.Item
                      label={
                        <LabelTips tips="返回数据的最大数量限制，一旦已保存的数据超过该值便不会再发起请求">
                          最大数据量
                        </LabelTips>
                      }
                      name="max_number_of_result_data"
                      rules={[{ required: true }]}
                      initialValue={1000}
                    >
                      <InputNumber placeholder="请输入" min={0} />
                    </Form.Item>
                  </Col>
                  <Col span={8}>
                    <Form.Item
                      label={
                        <LabelTips
                          tips={`返回数据中应检测的list的字段名，例如{"data": "result":[]}的键值是data.result`}
                        >
                          返回数据集合键值
                        </LabelTips>
                      }
                      name="filed_of_result_data"
                      rules={[{ required: true }]}
                    >
                      <Input placeholder="请输入" />
                    </Form.Item>
                  </Col>
                  <Col span={8}>
                    <Form.Item
                      label={
                        <LabelTips tips={`最大请求次数`}>
                          最大请求次数
                        </LabelTips>
                      }
                      name="max_count_of_request"
                      rules={[{ required: true }]}
                    >
                      <InputNumber placeholder="请输入" min={0}/>
                    </Form.Item>
                  </Col>
                </>
              ) : null
            }
          </Form.Item>

          <Col span={24}>
            <Form.Item
              label={<LabelTips tips="Request headers">请求头</LabelTips>}
            >
              <FormArrayList name="headers" />
            </Form.Item>
          </Col>

          <Col span={24}>
            <Form.Item
              label={
                <LabelTips tips="POST请求会以转换为json body发出， GET请求会转换为URL参数">
                  请求参数
                </LabelTips>
              }
            >
              <FormArrayList name="body" />
            </Form.Item>
          </Col>

          <Col span={24}>
            <Form.Item
              label={
                <LabelTips tips="对接收的数据按规则依次执行展开">
                  嵌套数据处理
                </LabelTips>
              }
            >
              <FormArrayList name="nested_config" isNestedConfig />
            </Form.Item>
          </Col>

          <Col span={24}>
            <Form.Item
              label={
                <LabelTips tips="key对应的值会转换为value对应的值">
                  参数转换规则
                </LabelTips>
              }
            >
              <FormArrayList name="map_rules" />
            </Form.Item>
          </Col>

          <Col span={24}>
            <Form.Item
              label={
                <LabelTips tips="配置数据库表中的列, key是列名，value是返回或转换后的数据中值的键名。">
                  列配置
                </LabelTips>
              }
              rules={[{ required: true }]}
              extra={"列配置变更后表结构会重新生成，旧表会被备份"}
            >
              <FormArrayList name="db_columns_config" isColumnConfig />
            </Form.Item>
          </Col>
          <Col span={24}>
            <Form.Item
              label={
                <LabelTips tips="例如：INSERT INTO table_name (column1, column2) VALUES (${data#id}, ${data#name})">
                  插入SQL&nbsp;
                  <Button type="primary" size="small" onClick={generateSql}>
                    点击生成
                  </Button>
                </LabelTips>
              }
              name="template_string"
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
          <Col span={24}>
            <Form.Item label="描述" name="desc">
              <Input.TextArea placeholder="请输入" />
            </Form.Item>
          </Col>
        </Row>
      </Form>
    </Drawer>
  );
}
