'use client'
import { Layout, theme } from 'antd'
import HeaderForm from './components/header-form';
import ContentTable from './components/content-table';

const { Header, Content } = Layout

export default function pages() {
    const {
        token: { colorBgContainer },
    } = theme.useToken();
    return (
        <Layout>
            <Header style={{backgroundColor: colorBgContainer, height: 52, padding: '12px 16px', borderLeft: '1px solid #f5f5f5'}}>
                <HeaderForm onAddClick={function (): void {
                    throw new Error('Function not implemented.');
                } } onSearch={function (name: string): void {
                    throw new Error('Function not implemented.');
                } } />
            </Header>
            <Content>
                <ContentTable  />
            </Content>
        </Layout>
    )
}