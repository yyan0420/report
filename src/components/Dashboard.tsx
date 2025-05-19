import Reports from "../app/Reports";
import Filters from "./Filters";

const Dashboard = () => {
    return (
        <div>
            <h3>Dashboard</h3>
            <Filters />
            <Reports />
        </div>
    )
};

export default Dashboard;
