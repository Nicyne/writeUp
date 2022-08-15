import { PageFooter } from 'components/pageFooter';
import { PageHeader } from 'components/pageHeader';
import { PropsWithChildren } from 'react';

export function DefaultLayout(props: PropsWithChildren) {
  return (
    <>
      <PageHeader />
      <main id="main">{props.children}</main>
      <PageFooter />
    </>
  );
}
