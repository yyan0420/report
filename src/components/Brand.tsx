import React from "react";
import { graphql, useLazyLoadQuery } from "react-relay";
import type { BrandQuery } from "../__generated__/BrandQuery.graphql";
import { Table } from "antd";
import type { ColumnsType } from "antd/es/table";
import { PercentageFormatter } from "./Helper";

type RecordType = NonNullable<BrandQuery["response"]["yoyTable"]>[number];

const columns: ColumnsType<RecordType> = [
  { title: "Name", dataIndex: "name", key: "name" },
  {
    title: "Total 4",
    dataIndex: "total1",
    key: "total1",
    render: (value, record) => (
      <div>
        {value}
        <PercentageFormatter pct={record.percentage1} />
      </div>
    ),
  },
  {
    title: "Total 3",
    dataIndex: "total2",
    key: "total2",
    render: (value, record) => (
      <div>
        {value}
        <PercentageFormatter pct={record.percentage2} />
      </div>
    ),
  },
  {
    title: "Total 2",
    dataIndex: "total3",
    key: "total3",
    render: (value, record) => (
      <div>
        {value}
        <PercentageFormatter pct={record.percentage3} />
      </div>
    ),
  },
  { title: "Total 1", dataIndex: "total4", key: "total4" },
];

const Brand: React.FC = () => {
  const data = useLazyLoadQuery<BrandQuery>(
    graphql`
      query BrandQuery {
        yoyTable {
          name
          code
          qty1
          total1
          qty2
          total2
          qty3
          total3
          qty4
          total4
          percentage1
          percentage2
          percentage3
          totalDiff1
          totalDiff2
          totalDiff3
        }
      }
    `,
    {}
  );

  return (
    <Table
      columns={columns}
      dataSource={data.yoyTable}
      rowKey="name"
      pagination={{ pageSize: 100 }}
      bordered
    />
  );
};

export default Brand;
