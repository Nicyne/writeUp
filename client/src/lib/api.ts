import { INote, INoteShallow, allowance } from 'types';

type endpoint = 'auth' | 'user' | 'note' | 'notes' | 'share';

interface IUserAllowance {
  userId: string;
  allowance: allowance;
}

interface IResponseArray extends Response {
  success: boolean;
  content: [];
  time: string;
  code?: string;
  message?: string;
}

interface IResponseObject<T> extends Omit<IResponseArray, 'content'> {
  content: T;
}

interface IRequestOptions {
  endpoint: endpoint;
  method: 'GET' | 'POST' | 'DELETE' | 'PUT';
  params?: string;
  body?: string;
  headers?: HeadersInit;
}

export class Api {
  private contentType = { 'Content-Type': 'application/json' };

  constructor() {}

  private async requestBuilder<T = IResponseArray | IResponseObject<any>>(
    options: IRequestOptions
  ) {
    const url =
      window.location.protocol + '//' + window.location.hostname + ':8080/api/'; //TODO Port shouldn't be static
    let params = options.params ? '/' + options.params : '';

    return new Promise<T>((resolve, reject) => {
      fetch(url + options.endpoint + params, {
        method: options.method,
        credentials: 'include',
        headers: {
          ...options.headers,
          ...this.contentType,
        },
        body: options.body,
      })
        .then((res) => res.json())
        .then((res) => resolve(res))
        .catch((err) => reject(err));
    });
  }

  /* Authentication */

  public async login(username: string, password: string) {
    const res = await this.requestBuilder({
      endpoint: 'auth',
      method: 'POST',
      body: JSON.stringify({ username: username, passwd: password }),
    });
    if (!res.success) throw new Error(res.code);
    return res.content;
  }

  public async logout() {
    const res = await this.requestBuilder({
      endpoint: 'auth',
      method: 'DELETE',
    });
    if (!res.success) throw new Error(res.code);
    return res.content;
  }

  public async getCurrentUser() {
    const res = await this.requestBuilder({
      endpoint: 'auth',
      method: 'GET',
    });
    if (!res.success) throw new Error(res.code);
    return res.content;
  }

  /* User */

  public async addUser(username: string, password: string) {
    if (!username || !password) throw new Error('received invalid credentials');
    const res = await this.requestBuilder({
      endpoint: 'user',
      method: 'POST',
      body: JSON.stringify({ username: username, passwd: password }),
    });
    if (!res.success) throw new Error(res.code);
    return res.content;
  }

  public async deleteUser() {
    const res = await this.requestBuilder({
      endpoint: 'user',
      method: 'DELETE',
    });
    if (!res.success) throw new Error(res.code);
    return res.content;
  }

  public async updateUser() {
    throw new Error('Not implemented.');
  }

  /* Notes */

  public async getNotes(): Promise<INoteShallow[]> {
    const res = await this.requestBuilder<IResponseArray>({
      endpoint: 'notes',
      method: 'GET',
    });
    if (!res.success) throw new Error(res.code);
    return <INoteShallow[]>res.content;
  }

  public async getNote(id: string): Promise<INote> {
    const res = await this.requestBuilder({
      endpoint: 'note',
      method: 'GET',
      params: id,
    });
    if (!res.success) throw new Error(res.code);
    return <INote>res.content;
  }

  public async addNote(
    title: string,
    content: string,
    tags: string[]
  ): Promise<INote> {
    const res = await this.requestBuilder({
      endpoint: 'note',
      method: 'POST',
      body: JSON.stringify({
        title,
        content,
        tags,
      }),
    });
    if (!res.success) throw new Error(res.code);
    return <INote>res.content;
  }

  public async deleteNote(id: string) {
    const res = await this.requestBuilder({
      endpoint: 'note',
      method: 'DELETE',
      params: id,
    });
    if (!res.success) throw new Error(res.code);
    return res.content;
  }

  public async updateNote(note: INote): Promise<INote> {
    note.note.tags = note.note.tags.filter((tag) => tag != '');
    const res = await this.requestBuilder({
      endpoint: 'note',
      method: 'PUT',
      params: note.note_id,
      body: JSON.stringify(note.note),
    });
    if (!res.success) throw new Error(res.code);
    return <INote>res.content;
  }

  /* SHARE */

  public async getShareToken(): Promise<string> {
    const res: IResponseObject<{ code: string }> = await this.requestBuilder({
      endpoint: 'share',
      method: 'GET',
    });
    if (!res.success) throw new Error(res.code);
    return res.content.code;
  }

  public async addRelation(code: string) {
    const res = await this.requestBuilder({
      endpoint: 'share',
      method: 'POST',
      body: JSON.stringify({
        code: code,
      }),
    });
    if (!res.success) throw new Error(res.code);
    return res.content;
  }

  public async deleteRelation(userId: string) {
    const res = await this.requestBuilder({
      endpoint: 'share',
      method: 'DELETE',
      params: userId,
    });
    if (!res.success) throw new Error(res.code);
    return res.content;
  }

  public async updateAllowances(nodeId: string, allowances: IUserAllowance[]) {
    const res = await this.requestBuilder({
      endpoint: 'share',
      method: 'PUT',
      body: JSON.stringify(allowances),
    });
    if (!res.success) throw new Error(res.code);
    return res.content;
  }
}

const defaultApi = new Api();
export default defaultApi;
