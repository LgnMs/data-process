import {useSWRConfig} from "swr";
import {Button, Col, Drawer, Form, Input, InputNumber, message, Radio, Row, Space} from "antd";
import React from "react";
import {CloseOutlined, MinusCircleOutlined, PlusOutlined} from "@ant-design/icons";
import FormItemLabelTips from "@/app/manage/components/form-item-label-tips";
import * as CollectConfig from "@/api/collect_config";
import {ICommonCollectionSettingProps} from "@/app/manage/collection-setting/page";
import { useMainContext } from "@/contexts/main";

interface IEditFormProps extends ICommonCollectionSettingProps {
    open: boolean
    close: () => void
}
export default function EditForm(props: IEditFormProps) {
    const { mutate } = useSWRConfig();
    const [form] = Form.useForm();
    const { state } = useMainContext()!;

    async function onSubmit() {
        const values = await form.validateFields();

        const headers: Record<string, string> = {}

        values.headers?.forEach((item: any) => {
            headers[item.key] = item.value
        })

        const map_rules = values.map_rules?.map((item: any) => [item.key, item.value])

        const body: Record<string, string> = {}
        values.body?.forEach((item: any) => {
            body[item.key] = item.value
        })

        const data = {
            ...values,
            headers,
            map_rules,
            body: JSON.stringify(body)
        }

        const res = await CollectConfig.add(data)

        if (res.data) {
            await mutate([CollectConfig.LIST, state.collectConfig.pagination])

            message.success("添加成功");

            close()
        }

    }

    function close() {
        props.close()
        form.resetFields()
    }

    return <Drawer
            title="采集配置"
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
        <Form layout="vertical"  form={form} labelAlign="left" labelWrap>
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
                            </>
                        ) : null
                    }
                </Form.Item>
                <Col span={24}>
                    <Form.Item label={<FormItemLabelTips tips="Request headers 目前post请求会自动设置：Content-Type: application/json">请求头</FormItemLabelTips>}>
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

function FormArrayList(props: { name: string, buttonText?: string }) {
    return <Form.List name={props.name}>
            {(fields, { add, remove }, { errors }) => (
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
            )}
        </Form.List>
}