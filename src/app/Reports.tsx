import { useState } from 'react';
import { Tabs, message } from 'antd';
import Brand from '../components/Brand';

const { TabPane } = Tabs;

const Reports = () => {
  // State to track the active tab
  const [activeTab, setActiveTab] = useState<string>('1');

  // Actions to handle when each tab is clicked
  const handleTabChange = (key: string) => {
    setActiveTab(key);
    switch (key) {
      case '1':
        message.info('Revenue tab clicked');
        break;
      case '2':
        message.info('Target tab clicked');
        break;
      case '3':
        message.info('Prediction tab clicked');
        break;
      default:
        break;
    }
  };

  return (
    <div>
      <Tabs activeKey={activeTab} onChange={handleTabChange}>
        <TabPane tab="Brand" key="1">
          <h3>Brand</h3>
          <Brand />
        </TabPane>
      </Tabs>
    </div>
  );
};

export default Reports;
