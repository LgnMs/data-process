import { QuestionCircleFilled } from "@ant-design/icons";
import { Space, Tooltip } from "antd";
import { ReactNode } from "react";

export default function FormItemLabelTips(props: {
  children: ReactNode;
  tips: string;
}) {
  return (
    <Tooltip title={props.tips}>
      <Space>
        <span>{props.children}</span>
        <QuestionCircleFilled rev={undefined} />
      </Space>
    </Tooltip>
  );
}
