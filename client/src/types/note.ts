export interface INote {
  note_id: string;
  note: {
    title: string;
    content: string;
    owner_id: string;
    tags: string[];
  };
  allowance: string;
}

export interface INoteShallow {
  note_id: string;
  title: string;
  tags: string[];
  allowance: string;
}
