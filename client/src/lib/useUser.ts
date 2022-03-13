import useSWR from 'swr';

const fetcher = () =>
  fetch('http://localhost:8080/api/user', {
    credentials: 'include',
  }).then((r) => {
    if (!r.ok) return { error: true };
    return r.json();
  });

export function useUser() {
  const { data, mutate } = useSWR('user', fetcher);
  // if data is not defined, the query has not completed
  const loading = !data;
  let user = null;
  if (!data?.error) user = data;
  return [user, { loading, mutate }];
}
