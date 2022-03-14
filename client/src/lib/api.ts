import { INote } from 'types';

type endpoints = 'auth' | 'user' | 'note' | 'notes';

export class Api {
  public readonly baseUrl: string;
  private contentType = { 'Content-Type': 'application/json' };

  constructor(url: string) {
    if (!url) throw new Error('Invalid base URL');
    if (!url.endsWith('/')) url += '/';
    this.baseUrl = url;
  }

  private async requestBuilder(
    endpoint: endpoints,
    param?: string,
    options?: RequestInit
  ) {
    const requestOptions = options
      ? options
      : ({ method: 'GET', credentials: 'include' } as RequestInit);
    const requestParam = param ? '/' + param : '';

    return new Promise<any>((resolve, reject) => {
      fetch(this.baseUrl + endpoint + requestParam, requestOptions)
        .then((res) => resolve(res))
        .catch((err) => reject(err));
    });
  }

  /* Authentication */

  public async login(username: string, password: string) {
    const res = await this.requestBuilder('auth', undefined, {
      method: 'POST',
      headers: this.contentType,
      credentials: 'include',
      body: JSON.stringify({ username: username, passwd: password }),
    });
    if (!res.ok) throw new Error('Could not authenticate');
    return res.json();
  }

  public async logout() {
    const res = await this.requestBuilder('auth', undefined, {
      method: 'DELETE',
      credentials: 'include',
    });
    if (!res.ok) throw new Error('Could not logout');
    return res.json();
  }

  /* User */
  public async getCurrentUser() {
    const res = await this.requestBuilder('user');
    if (!res.ok) throw new Error('Could not get current User');
    return res.json();
  }

  /* Notes */

  public async getNotes(): Promise<INote[]> {
    const res = await this.requestBuilder('notes');
    if (!res.ok) return [];
    return <INote[]>res.json();
  }

  public async getNote(id: string): Promise<INote> {
    const res = await this.requestBuilder('note', id);
    if (!res.ok) throw new Error('Could not fetch note with id: ' + id);
    return <INote>res.json();
  }

  public async saveNote(note: INote): Promise<INote> {
    const res = await this.requestBuilder('note', note.note_id, {
      method: 'PUT',
      headers: this.contentType,
      credentials: 'include',
      body: JSON.stringify(note.note),
    });
    if (!res.ok)
      throw new Error('Could not save note with id: ' + note.note_id);
    return <INote>res.json();
  }
}

const defaultApi = new Api('http://localhost:8080/api');
export default defaultApi;