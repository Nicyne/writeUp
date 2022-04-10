import sha256 from 'crypto-js/sha256';
import Base64 from 'crypto-js/enc-base64';

export function getHash(string: string): string {
  return Base64.stringify(sha256(string));
}
