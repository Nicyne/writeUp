import { NextPage } from 'next';
import PageHeader from '../pageHeader';

const MainLayout: NextPage = ({ children }) => {
  return (
    <>
      <PageHeader />
      <main>{children}</main>
    </>
  );
};

export default MainLayout;
