import { NextApiRequest, NextApiResponse } from 'next';

export default function handler(req: NextApiRequest, res: NextApiResponse) {
  const url = process.env.API_URL;
  res.status(200).json({
    url: url,
  });
}
