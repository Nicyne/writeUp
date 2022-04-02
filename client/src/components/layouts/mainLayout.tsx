import { NextPage } from 'next';
import { PageHeader } from 'components';

const MainLayout: NextPage = ({ children }) => {
  return (
    <>
      <PageHeader />
      <main>{children}</main>
    </>
  );
};

export default MainLayout;
