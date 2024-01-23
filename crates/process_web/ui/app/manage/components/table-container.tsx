import React from "react";
import styles from "./table-container.module.scss";

export default function TableContainer({
  children,
}: {
  children: React.ReactNode;
}) {
  console.log(styles);
  return (
    <div className={styles.container}>
      <div className={styles.content}>
        <div className={styles.body}>{children}</div>
      </div>
    </div>
  );
}
