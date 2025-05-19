import React, { useState } from "react";
import { Button, Drawer, Select, type SelectProps, Space } from "antd";
import { graphql, useLazyLoadQuery } from 'react-relay';
import { type FilterDrawerQuery } from '../__generated__/FilterDrawerQuery.graphql';

const FilterDrawer: React.FC = () => {
  const data = useLazyLoadQuery<FilterDrawerQuery>(
    graphql`
      query FilterDrawerQuery {
        brands(first: 1000) {
          edges {
            node {
              id
              name
              urlSlug
              privateLabel
              status
            }
          }
        }
      }
    `,
    {}
  );

  const options: SelectProps['options'] = data.brands.edges?.map(({ node }) => ({
    label: node.name,
    value: node.id,
  })) ?? [];

  const [open, setOpen] = useState(false);

  const showDrawer = () => {
    setOpen(true);
  };

  const closeDrawer = () => {
    setOpen(false);
  };

  const handleSelectChange = (value: string, field: string) => {
    console.log(`${field}:`, value);
  };

  return (
    <>
      <Button type="primary" onClick={showDrawer}>
        Filters
      </Button>

      <Drawer
        title="More Filters"
        placement="right"
        onClose={closeDrawer}
        open={open}
        width={300}
      >
        <Space direction="vertical" style={{ width: "100%" }}>
          <Select
            placeholder="Select Brand(s)"
            onChange={(value) => handleSelectChange(value, "region")}
            mode='multiple'
            style={{ width: '80%' }}
            allowClear
            options={options}
            filterOption={(input, option) =>
              (option?.label as string)?.toLowerCase().includes(input.toLowerCase())
            }
          />

          {/* <Select
            placeholder="Select Channel"
            onChange={(value) => handleSelectChange(value, "channel")}
          >
            <Option value="online">Online</Option>
            <Option value="retail">Retail</Option>
            <Option value="wholesale">Wholesale</Option>
          </Select> */}
        </Space>
      </Drawer>
    </>
  );
};

export default FilterDrawer;
