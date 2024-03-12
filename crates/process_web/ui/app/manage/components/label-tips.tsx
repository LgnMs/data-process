import { QuestionCircleFilled } from "@ant-design/icons";
import { Space, Tooltip } from "antd";
import { ReactNode } from "react";

export default function LabelTips(props: {
  children: ReactNode;
  tips: string;
}) {
  return (
    <Tooltip title={props.tips}>
      <Space>
        <span>{props.children}</span>
        <QuestionCircleFilled />
      </Space>
    </Tooltip>
  );
}
