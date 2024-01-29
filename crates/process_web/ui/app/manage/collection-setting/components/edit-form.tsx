import {useSWRConfig} from "swr";
import { Button, Col, Drawer, Form, Input, InputNumber, message, Radio, Row, Space } from "antd";
import React, { useEffect, useState } from "react";
import {CloseOutlined, PlusOutlined} from "@ant-design/icons";
import FormItemLabelTips from "@/app/manage/components/form-item-label-tips";
import * as CollectConfig from "@/api/collect_config";
import {ICommonCollectionSettingProps} from "@/app/manage/collection-setting/page";
import { useMainContext } from "@/contexts/main";
import { clone } from "lodash";

interface IEditFormProps extends ICommonCollectionSettingProps {
    open: boolean
    close: () => void
}
export default function EditForm(props: IEditFormProps) {
    const { mutate } = useSWRConfig();
    const [form] = Form.useForm();
    const { state, dispatch } = useMainContext()!;
    const [mode, setMode] = useState<'edit'|'add'>('add');

    async function onSubmit() {
        await form.validateFields();
        const values = form.getFieldsValue(true)

        console.log(values)
        const headers: Record<string, string> = {}

        values.headers?.forEach((item: any) => {
            headers[item.key] = item.value
        })

        const map_rules = values.map_rules?.map((item: any) => [item.key, item.value])

        const body: Record<string, string | number | boolean> = {}
        values.body?.forEach((item: any) => {
            if (Number.isNaN(Number(item.value))) {
                if (item.value === 'true') {
                    body[item.key] = true
                } else if (item.value === 'false') {
                    body[item.key] = false
                } else {
                    body[item.key] = item.value
                }
            } else {
                body[item.key] = Number(item.value)
            }

        })

        const data = {
            ...values,
            headers,
            map_rules,
            body: JSON.stringify(body)
        }


        let res;
        if (mode === 'add') {
            res = await CollectConfig.add(data)
        } else {
            res = await CollectConfig.update_by_id(data.id, data)
        }

        if (res.data) {
            await mutate([CollectConfig.LIST, state.collectConfig.pagination])

            message.success("操作成功");

            close()
        }

    }

    function close() {
        props.close()
        form.resetFields()
        dispatch({
            type: 'collectConfig.setEditFormData',
            editFormData: null
        })
    }

    useEffect(() => {
        if (state.collectConfig.editFormOpen) {
            if (state.collectConfig.editFormData) {

                const data: any = clone(state.collectConfig.editFormData);

                if (data.headers) {
                    data.headers = Object.keys(data.headers).map((key) => {
                        return { key, value: data.headers[key] }
                    })
                }
                if (data.body) {
                    const obj = JSON.parse(data.body);
                    data.body =  Object.keys(obj).map(key => {
                        return { key, value: obj[key] }
                    })
                }
                if (data.map_rules) {
                    data.map_rules =  data.map_rules.map((item: Array<string>) => {
                        return { key: item[0], value: item[1] }
                    })
                }

                form.setFieldsValue(data)
                setMode('edit')
            } else {
                setMode('add')
            }
        }

    }, [state.collectConfig.editFormOpen])

    function setDefaultForPostHeaders(method: "GET" | "POST") {
        let headers: Array<{key: string, value: string}> | undefined = form.getFieldValue("headers");
        if (headers === undefined) {
            headers = [];
        }

        const jsonHeaderIndex = headers.findIndex(header => header.key === "Content-Type" && header.value === "application/json");

        if (jsonHeaderIndex === -1 && method === "POST") {
            headers.push({key: "Content-Type", value: "application/json"})
        }

        if (jsonHeaderIndex > -1 && method === "GET") {
            headers.splice(jsonHeaderIndex, 1);
        }

        form.setFieldValue("headers", headers);
    }

    return <Drawer
            title={`${mode === 'add' ? '新增' : '编辑'}采集配置`}
            open={props.open}
            width={800}
            extra={
                <Space>
                    <Button onClick={close}>取消</Button>
                    <Button onClick={onSubmit} type="primary">提交</Button>
                </Space>
            }
            onClose={close}
    >
        <Form layout="vertical"
          form={form}
          labelAlign="left"
          labelWrap
          onFieldsChange={(changedFields) => {
              changedFields.forEach(item => {
                  if (item.name[0] === "method") {
                      setDefaultForPostHeaders(item.value)
                  }
              })
          }}
        >
            <Row gutter={16}>
                <Col span={8}>
                    <Form.Item label="名称" name="name" rules={[{ required: true }]}>
                        <Input placeholder='请输入' />
                    </Form.Item>
                </Col>
                <Col span={8}>
                    <Form.Item label="请求类型" name="method" rules={[{ required: true }]}>
                        <Radio.Group>
                            <Radio value="GET">GET</Radio>
                            <Radio value="POST">POST</Radio>
                        </Radio.Group>
                    </Form.Item>
                </Col>
                <Col span={8}>
                    <Form.Item label="缓存表" name="cache_table_name" rules={[{ required: true }]}>
                        <Input placeholder='请输入' />
                    </Form.Item>
                </Col>
                <Col span={24}>
                    <Form.Item label="请求地址(url)" name="url" rules={[{ required: true }]}>
                        <Input placeholder='请输入' />
                    </Form.Item>
                </Col>
                <Col span={8}>
                    <Form.Item label={<FormItemLabelTips tips="用于分页接口获取">是否循环请求</FormItemLabelTips>} name="loop_request_by_pagination">
                        <Radio.Group>
                            <Radio value={false}>否</Radio>
                            <Radio value={true}>是</Radio>
                        </Radio.Group>
                    </Form.Item>
                </Col>
                <Form.Item
                    noStyle
                    shouldUpdate={(prevValues:any, currentValues:any) => prevValues['loop_request_by_pagination'] !== currentValues['loop_request_by_pagination']}
                >
                    {({ getFieldValue }) =>
                        getFieldValue('loop_request_by_pagination') === true ? (
                            <>
                                <Col span={8}>
                                    <Form.Item
                                        label={<FormItemLabelTips tips="请求参数中的页码字段">页码key</FormItemLabelTips>}
                                        name="current_key"
                                        rules={[{ required: true }]}
                                    >

                                        <Input placeholder="请输入"/>
                                    </Form.Item>
                                </Col>
                                <Col span={8}>
                                    <Form.Item
                                        label={<FormItemLabelTips tips="请求参数中的分页大小字段">分页大小key</FormItemLabelTips>}
                                        name="page_size_key"
                                        rules={[{ required: true }]}
                                    >
                                        <Input placeholder="请输入"/>
                                    </Form.Item>
                                </Col>
                                <Col span={8}>
                                    <Form.Item
                                        label={<FormItemLabelTips tips="返回数据的最大数量限制，一旦已保存的数据超过该值便不会再发起请求">最大数据量</FormItemLabelTips>}
                                        name="max_number_of_result_data"
                                        rules={[{ required: true }]}
                                        initialValue={100}
                                    >
                                        <InputNumber placeholder="请输入"/>
                                    </Form.Item>
                                </Col>
                                <Col span={8}>
                                    <Form.Item
                                        label={<FormItemLabelTips tips={`返回数据中应检测的list的字段名，例如{"data": "result":[]}的键值是data.result`}>返回数据集合键值</FormItemLabelTips>}
                                        name="filed_of_result_data"
                                        rules={[{ required: true }]}
                                    >
                                        <Input placeholder="请输入"/>
                                    </Form.Item>
                                </Col>
                                <Col span={8}>
                                    <Form.Item
                                        label={<FormItemLabelTips tips={`最大请求次数`}>最大请求次数</FormItemLabelTips>}
                                        name="max_count_of_request"
                                        rules={[{ required: true }]}
                                    >
                                        <InputNumber placeholder="请输入"/>
                                    </Form.Item>
                                </Col>
                            </>
                        ) : null
                    }
                </Form.Item>

                <Col span={24}>
                    <Form.Item label={<FormItemLabelTips tips="Request headers">请求头</FormItemLabelTips>}>
                        <FormArrayList name="headers" />
                    </Form.Item>
                </Col>

                <Col span={24}>
                    <Form.Item label={<FormItemLabelTips tips="POST请求会以转换为json body发出， GET请求会转换为URL参数">请求参数</FormItemLabelTips>}>
                        <FormArrayList name="body" />
                    </Form.Item>
                </Col>

                <Col span={24}>
                    <Form.Item label={<FormItemLabelTips tips="key对应的值会转换为value对应的值">参数转换规则</FormItemLabelTips>}>
                        <FormArrayList name="map_rules" />
                    </Form.Item>
                </Col>
                <Col span={24}>
                    <Form.Item
                        label={<FormItemLabelTips tips="例如：INSERT INTO table_name (column1, column2) VALUES (${data#id}, ${data#name})">导出字符模板</FormItemLabelTips>}
                        name="template_string"
                        rules={[{ required: true }]}
                    >
                        <Input.TextArea placeholder="请输入" />
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
}

function FormArrayList(props: { name: string, buttonText?: string, initialValue?: any[] }) {
    return <Form.List name={props.name} initialValue={props.initialValue}>
            {(fields, { add, remove }, { errors }) => {
                return (
                      <div style={{display: 'flex', flexDirection: 'column', rowGap: 16}}>
                        {fields.map((field, index) => (
                            <Space key={field.key}>
                                <Form.Item noStyle name={[field.name, 'key']}>
                                    <Input placeholder="key" />
                                </Form.Item>
                                <Form.Item noStyle name={[field.name, 'value']}>
                                    <Input placeholder="value" />
                                </Form.Item>
                                <CloseOutlined
                                    onClick={() => {
                                        remove(field.name);
                                    }}
                                    rev={undefined}
                                />
                            </Space>
                        ))}
                        <Button
                            type="dashed"
                            onClick={() => add()}
                            style={{width: '380px'}}
                            icon={<PlusOutlined rev={undefined}/>}
                        >
                            {props.buttonText ? props.buttonText : '添加'}
                        </Button>
                        <Form.ErrorList errors={errors}/>
                    </div>
                )
            }}
        </Form.List>
}