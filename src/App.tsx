import TabsMenu from './components/TabsMenu';
import BrandList from './BrandList';

const App = () => {
  return (
    <div style={{ minHeight: '100vh', width: '100%', padding: 20 }}>
      <h1>Dashboard</h1>
      <TabsMenu />
      <BrandList />
    </div>
  );
};

export default App;
