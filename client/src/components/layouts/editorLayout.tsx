import { PageHeader } from 'components';
import { PropsWithChildren } from 'react';

export function EditorLayout(props: PropsWithChildren) {
  return (
    <>
      <PageHeader />
      <main id="main">{props.children}</main>
    </>
  );
}
