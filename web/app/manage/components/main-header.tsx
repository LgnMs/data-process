import { Avatar, Dropdown, Layout, Space } from 'antd'
import Image from 'next/image'
import { UserOutlined, LogoutOutlined } from '@ant-design/icons';
import styles from './main.module.scss'
import { useMainContext } from '@/contexts/main';

const { Header } = Layout;

export function MainHeader() {
  const { state } = useMainContext()!
  
  const { dispatch } = useMainContext()!;
  
  const namespace = 'main-header'
  return <Header style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between', position: 'fixed', top: 0, left: 0, width: '100vw', zIndex: 999 }}>
    <div className={styles[`${namespace}-logo`]}>
      {/* <Image
        src="/images/logo.png"
        width={32}
        height={32}
        alt="logo"
      /> */}
      <span className={styles[`${namespace}-logo-title`]}>数据处理中心</span>
    </div>
    {
      <Space style={{color: '#fff'}}>
        <Dropdown
          menu={{
            items: [
              {
                key: '1',
                label: <Space><LogoutOutlined rev={undefined} />注销</Space>,
                onClick: () => { 
                  dispatch({ type: 'setToken', token: '' })
                  dispatch({ type: 'setRoles', roles: [] })
                  dispatch({ type: 'setPermissions', permissions: [] })
                  // logout()
                }
              }
            ]
          }}
          arrow
        >
          <div>
            <Avatar icon={<UserOutlined rev={undefined} />} />
            <span style={{display: 'inline-block', verticalAlign: '-3px'}}>{state.userInfo?.nickName || state.userInfo?.userName}</span>
          </div>
        </Dropdown>
      </Space>
    }
  </Header>
}