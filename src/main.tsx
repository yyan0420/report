import { StrictMode } from 'react';
import { createRoot } from 'react-dom/client';
import { RelayEnvironmentProvider } from 'react-relay';
import { ConfigProvider } from 'antd';
import environment from './relay/relayEnvironment.ts';;
import LayoutComponents from './app/LayoutComponents.tsx';

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
        <LayoutComponents />
      </ConfigProvider>
    </RelayEnvironmentProvider>
  </StrictMode>
);
