import { useEffect, useState } from 'react';

export function NotFound() {
  const [pathname, setPathname] = useState('');

  useEffect(() => {
    if (typeof window === 'undefined') return;

    setPathname(window.location.pathname);
  }, []);

  return (
    <>
      <h1>
        Page <span>{pathname}</span> not found
      </h1>
    </>
  );
}

export default NotFound;
