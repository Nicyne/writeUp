import { useMountEffect } from 'hooks';
import { createContext, PropsWithChildren, useState } from 'react';

export interface ISystemContext {
  version: string;
}

export const SystemContext = createContext<ISystemContext>({
  version: '',
});

export const SystemContextProvider = (props: PropsWithChildren) => {
  const [version, setVersion] = useState<string>('');

  useMountEffect(() => {
    fetch('/api/system')
      .then((res) => res.json())
      .then((res) => {
        if (!res.success) {
          console.error(res.message);
          return;
        }

        if (!res.content.version) {
          console.error('response has no key called "version"');
          return;
        }

        setVersion('v.' + res.content.version);
      })
      .catch((err) => console.error(err));
  });

  return (
    <SystemContext.Provider value={{ version }}>
      {props.children}
    </SystemContext.Provider>
  );
};
