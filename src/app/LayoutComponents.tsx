import { Tabs } from "antd";
import Dashboard from "../components/Dashboard";

const { TabPane } = Tabs;

const LayoutComponents = () => {
  return (
    <div style={{ width: "100vw", overflow: "hidden" }}>
      <Tabs defaultActiveKey="1" type="line" style={{ height: "100%" }}>
        <TabPane tab="Dashboard" key="1">
          <Dashboard />
        </TabPane>
        <TabPane tab="Users" key="2">
          <div style={{ height: "100%", backgroundColor: "#e6f7ff" }}>
            Users
          </div>
        </TabPane>
        <TabPane tab="Groups" key="3">
          <div style={{ height: "100%", backgroundColor: "#e6f7ff" }}>
            Groups
          </div>
        </TabPane>
      </Tabs>
    </div>
  );
};

export default LayoutComponents;
