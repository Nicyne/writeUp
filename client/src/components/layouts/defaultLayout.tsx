import { PropsWithChildren } from 'react';

import { PageHeader } from 'components/pageHeader';

export function DefaultLayout(props: PropsWithChildren) {
  return (
    <>
      <PageHeader />
      {props.children}
    </>
  );
}
