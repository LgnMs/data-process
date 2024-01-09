'use client'
import { ConfigProvider, Layout } from 'antd';
import { useState } from 'react';
import zhCN from 'antd/locale/zh_CN';
import { MainHeader } from './main-header';
import { MainSider } from './main-sider';
import { MainContent } from './main-content';
import dayjs from 'dayjs'
import quarterOfYear from 'dayjs/plugin/quarterOfYear'
import 'dayjs/locale/zh-cn';
import { MainContextProvider } from '@/contexts/main';

dayjs.extend(quarterOfYear)

export default function MainLayout({
  children,
}: {
  children: React.ReactNode,
}) {
  const [collapsed, setCollapsed] = useState(false);
  
  return <MainContextProvider>
    <ConfigProvider locale={zhCN}>
      <Layout style={{position: 'relative'}}>
        <MainHeader />
        <Layout hasSider style={{position: 'relative', padding: `64px 0 0 ${collapsed ? '80px' : '200px'}`}}>
          <MainSider onCollapse={(value) => setCollapsed(value)}/>
          <Layout>
            {/* <MainBreadcrumb /> */}
            <MainContent>
              {children}
            </MainContent>
          </Layout>
        </Layout>
      </Layout>
    </ConfigProvider>
  </MainContextProvider>
}