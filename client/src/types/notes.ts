export type allowance = 'Owner' | 'ReadWrite' | 'Read' | 'Forbidden';

export interface INote {
  note_id: string;
  note: {
    title: string;
    content: string;
    owner_id: string;
    tags: string[];
  };
  allowance: allowance;
}

export interface INoteShallow {
  note_id: string;
  title: string;
  tags: string[];
  allowance: allowance;
}
