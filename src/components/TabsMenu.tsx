import { useState } from 'react';
import { Tabs, message } from 'antd';

const { TabPane } = Tabs;

const TabsMenu = () => {
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
        <TabPane tab="Revenue" key="1">
          <h3>Revenue Content</h3>
          {/* Content for the Revenue tab */}
        </TabPane>
        <TabPane tab="Target" key="2">
          <h3>Target Content</h3>
          {/* Content for the Target tab */}
        </TabPane>
        <TabPane tab="Prediction" key="3">
          <h3>Prediction Content</h3>
          {/* Content for the Prediction tab */}
        </TabPane>
      </Tabs>
    </div>
  );
};

export default TabsMenu;
