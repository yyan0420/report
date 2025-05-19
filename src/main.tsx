import { StrictMode } from 'react';
import { createRoot } from 'react-dom/client';
import { RelayEnvironmentProvider } from 'react-relay';
import { ConfigProvider } from 'antd';
import environment from './relay/relayEnvironment.ts';;
import App from './App.tsx';

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <RelayEnvironmentProvider environment={environment}>
      <ConfigProvider
        theme={{
          token: {
            colorPrimary: '#cb0000',
          },
        }}
      >
        <App />
      </ConfigProvider>
    </RelayEnvironmentProvider>
  </StrictMode>
);
