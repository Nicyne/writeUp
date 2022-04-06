import { INote, INoteShallow, allowance } from 'types';

type endpoint = 'auth' | 'user' | 'note' | 'notes' | 'share';

interface IUserAllowance {
  userId: string;
  allowance: allowance;
}

export class Api {
  private contentType = { 'Content-Type': 'application/json' };

  constructor() {}

  private async requestBuilder(
    endpoint: endpoint,
    param?: string,
    options?: RequestInit
  ) {
    const requestOptions = options
      ? options
      : ({ method: 'GET', credentials: 'include' } as RequestInit);
    const requestParam = param ? '/' + param : '';

    return new Promise<any>((resolve, reject) => {
      const url =
        window.location.protocol +
        '//' +
        window.location.hostname +
        ':8080/api/'; //TODO Port shouldn't be static
      console.log(url);

      fetch(url + endpoint + requestParam, requestOptions)
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

  public async addUser(username: string, password: string) {
    if (!username || !password) throw new Error('received invalid credentials');
    const res = await this.requestBuilder('user', undefined, {
      method: 'POST',
      credentials: 'include',
      headers: this.contentType,
      body: JSON.stringify({
        username: username,
        passwd: password,
      }),
    });
    if (!res.ok) throw new Error(res.error);
    return res.json();
  }

  public async deleteUser() {
    const res = await this.requestBuilder('user', undefined, {
      method: 'DELETE',
      credentials: 'include',
    });
    if (!res.ok) throw new Error(res.error);
    return res.json();
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

  public async getShareToken(): Promise<string> {
    const res = await this.requestBuilder('share');
    if (!res.ok) throw new Error(res.error);
    const data = await res.json();
    return data.code;
  }

  public async addRelation(token: string) {
    const res = await this.requestBuilder('share', undefined, {
      method: 'POST',
      credentials: 'include',
      headers: this.contentType,
      body: JSON.stringify({
        code: token,
      }),
    });
    const data = await res.json();
    if (!res.ok) throw data.error;
    return data;
  }

  public async deleteRelation(userId: string) {
    const res = await this.requestBuilder('share', userId, {
      method: 'DELETE',
      credentials: 'include',
    });
    const data = await res.json();
    if (!res.ok) throw data.error;
    return data;
  }

  public async updateAllowances(nodeId: string, allowances: IUserAllowance[]) {
    const res = await this.requestBuilder('share', undefined, {
      method: 'PUT',
      credentials: 'include',
      headers: this.contentType,
      body: JSON.stringify(allowances),
    });
    if (!res.ok) throw new Error(res.error);
    return res.json();
  }
}

const defaultApi = new Api();
export default defaultApi;
