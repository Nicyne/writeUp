import { PageHeader } from 'components/pageHeader';
import { PropsWithChildren } from 'react';

export function DefaultLayout(props: PropsWithChildren) {
  return (
    <>
      <PageHeader />
      {props.children}
    </>
  );
}
