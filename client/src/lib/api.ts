import { INote, INoteShallow } from 'types';

type endpoints = 'auth' | 'user' | 'note' | 'notes';

export class Api {
  public readonly baseUrl: string;
  private contentType = { 'Content-Type': 'application/json' };

  constructor(url: string) {
    if (!url) throw new Error('Invalid base URL');
    if (!url.startsWith('https')) console.warn('Api-Url is not using https.');
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

  public async addUser() {
    throw new Error('Not implemented.');
  }

  public async deleteUser() {
    throw new Error('Not implemented.');
  }

  public async updateUser() {
    throw new Error('Not implemented.');
  }

  /* Notes */

  public async getNotes(): Promise<INoteShallow[]> {
    const res = await this.requestBuilder('notes');
    if (!res.ok) return [];
    return <INoteShallow[]>res.json();
  }

  public async getNote(id: string): Promise<INote> {
    const res = await this.requestBuilder('note', id);
    if (!res.ok) throw new Error('Could not fetch note with id: ' + id);
    return <INote>res.json();
  }

  public async addNote(
    title: string,
    content: string,
    tags: string[]
  ): Promise<INote> {
    const res = await this.requestBuilder('note', undefined, {
      method: 'POST',
      headers: this.contentType,
      credentials: 'include',
      body: JSON.stringify({ title, content, tags }),
    });
    if (!res.ok) throw new Error('Could not create note');
    return <INote>res.json();
  }

  public async deleteNote(id: string) {
    const res = await this.requestBuilder('note', id, {
      method: 'DELETE',
      credentials: 'include',
    });
    if (!res.ok) throw new Error('Could not delete note');
    return res.json();
  }

  public async updateNote(note: INote): Promise<INote> {
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
