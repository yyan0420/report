import React from "react";
import { DatePicker, Select, Space } from "antd";
import type { RangePickerProps } from "antd/es/date-picker";
import FilterDrawer from "./FilterDrawer";

const { RangePicker } = DatePicker;
const { Option } = Select;

const Filters: React.FC = () => {
  const handleDateChange: RangePickerProps['onChange'] = (dates, dateStrings) => {
    console.log("Selected Dates: ", dateStrings);
  };

  const handleSelectChange = (value: string, field: string) => {
    console.log(`Selected ${field}:`, value);
  };

  return (
    <Space direction="vertical" size="middle" wrap>
      <RangePicker onChange={handleDateChange} />

      <Select
        placeholder="Select Brand Origin"
        style={{ width: 160 }}
        onChange={(value) => handleSelectChange(value, 'category')}
      >
        <Option value="All">All</Option>
        <Option value="Import">Import</Option>
        <Option value="Local">Local</Option>
      </Select>

      <Select
        placeholder="Select Datasource"
        style={{ width: 160 }}
        onChange={(value) => handleSelectChange(value, 'status')}
      >
        <Option value="Online + In Store">Online + In Store</Option>
        <Option value="In Store">In Store</Option>
      </Select>

      <FilterDrawer />
    </Space>
  );
};

export default Filters;
