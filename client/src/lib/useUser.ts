import useSWR from 'swr';
import { dApi } from 'lib';

const fetcher = () =>
  dApi.getCurrentUser().catch((err) => {
    return { error: true };
  });

export function useUser() {
  const { data, mutate } = useSWR('user', fetcher);
  // if data is not defined, the query has not completed
  const loading = !data;
  let user = null;
  if (!data?.hasOwnProperty('error')) user = data;
  return [user, { loading, mutate }];
}
